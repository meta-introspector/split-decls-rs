/* FP:hir.rs-0001 */ // ignore-tidy-filelength
/* FP:hir.rs-0002 */ use std::borrow::Cow;
/* FP:hir.rs-0003 */ use std::fmt;
/* FP:hir.rs-0004 */ 
/* FP:hir.rs-0005 */ use crate::rustc_abi::ExternAbi;
/* FP:hir.rs-0006 */ use crate::rustc_complete::attr::AttributeExt;
/* FP:hir.rs-0007 */ use crate::rustc_complete::token::CommentKind;
/* FP:hir.rs-0008 */ use crate::rustc_complete::util::parser::ExprPrecedence;
/* FP:hir.rs-0009 */ use crate::rustc_complete::{
/* FP:hir.rs-0010 */     self as ast, FloatTy, InlineAsmOptions, InlineAsmTemplatePiece, IntTy, Label, LitIntType,
/* FP:hir.rs-0011 */     LitKind, TraitObjectSyntax, UintTy, UnsafeBinderCastKind, join_path_idents,
/* FP:hir.rs-0012 */ };
/* FP:hir.rs-0013 */ pub use crate::rustc_complete::{
/* FP:hir.rs-0014 */     AssignOp, AssignOpKind, AttrId, AttrStyle, BinOp, BinOpKind, BindingMode, BorrowKind,
/* FP:hir.rs-0015 */     BoundConstness, BoundPolarity, ByRef, CaptureBy, DelimArgs, ImplPolarity, IsAuto,
/* FP:hir.rs-0016 */     MetaItemInner, MetaItemLit, Movability, Mutability, UnOp,
/* FP:hir.rs-0017 */ };
/* FP:hir.rs-0018 */ use crate::rustc_data_structures::fingerprint::Fingerprint;
/* FP:hir.rs-0019 */ use crate::rustc_data_structures::sorted_map::SortedMap;
/* FP:hir.rs-0020 */ use crate::rustc_data_structures::tagged_ptr::TaggedRef;
/* FP:hir.rs-0021 */ use crate::rustc_error_messages::{DiagArgValue, IntoDiagArg};
/* FP:hir.rs-0022 */ use crate::rustc_index::IndexVec;
/* FP:hir.rs-0023 */ use rustc_macros::{Decodable, Encodable, HashStable_Generic};
/* FP:hir.rs-0024 */ use crate::rustc_complete::def_id::LocalDefId;
/* FP:hir.rs-0025 */ use crate::rustc_complete::source_map::Spanned;
/* FP:hir.rs-0026 */ use crate::rustc_complete::{BytePos, DUMMY_SP, ErrorGuaranteed, Ident, Span, Symbol, kw, sym};
/* FP:hir.rs-0027 */ use crate::rustc_target::asm::InlineAsmRegOrRegClass;
/* FP:hir.rs-0028 */ use smallvec::SmallVec;
/* FP:hir.rs-0029 */ use thin_vec::ThinVec;
/* FP:hir.rs-0030 */ use tracing::debug;
/* FP:hir.rs-0031 */ 
/* FP:hir.rs-0032 */ use crate::LangItem;
/* FP:hir.rs-0033 */ use crate::attrs::AttributeKind;
/* FP:hir.rs-0034 */ use crate::def::{CtorKind, DefKind, MacroKinds, PerNS, Res};
/* FP:hir.rs-0035 */ use crate::def_id::{DefId, LocalDefIdMap};
/* FP:hir.rs-0036 */ pub(crate) use crate::hir_id::{HirId, ItemLocalId, ItemLocalMap, OwnerId};
/* FP:hir.rs-0037 */ use crate::intravisit::{FnKind, VisitorExt};
/* FP:hir.rs-0038 */ use crate::lints::DelayedLints;
/* FP:hir.rs-0039 */ 
/* FP:hir.rs-0040 */ #[derive(Debug, Copy, Clone, PartialEq, Eq, HashStable_Generic)]
/* FP:hir.rs-0041 */ pub enum AngleBrackets {
/* FP:hir.rs-0042 */     /// E.g. `Path`.
/* FP:hir.rs-0043 */     Missing,
/* FP:hir.rs-0044 */     /// E.g. `Path<>`.
/* FP:hir.rs-0045 */     Empty,
/* FP:hir.rs-0046 */     /// E.g. `Path<T>`.
/* FP:hir.rs-0047 */     Full,
/* FP:hir.rs-0048 */ }
/* FP:hir.rs-0049 */ 
/* FP:hir.rs-0050 */ #[derive(Debug, Copy, Clone, PartialEq, Eq, HashStable_Generic)]
/* FP:hir.rs-0051 */ pub enum LifetimeSource {
/* FP:hir.rs-0052 */     /// E.g. `&Type`, `&'_ Type`, `&'a Type`, `&mut Type`, `&'_ mut Type`, `&'a mut Type`
/* FP:hir.rs-0053 */     Reference,
/* FP:hir.rs-0054 */ 
/* FP:hir.rs-0055 */     /// E.g. `ContainsLifetime`, `ContainsLifetime<>`, `ContainsLifetime<'_>`,
/* FP:hir.rs-0056 */     /// `ContainsLifetime<'a>`
/* FP:hir.rs-0057 */     Path { angle_brackets: AngleBrackets },
/* FP:hir.rs-0058 */ 
/* FP:hir.rs-0059 */     /// E.g. `impl Trait + '_`, `impl Trait + 'a`
/* FP:hir.rs-0060 */     OutlivesBound,
/* FP:hir.rs-0061 */ 
/* FP:hir.rs-0062 */     /// E.g. `impl Trait + use<'_>`, `impl Trait + use<'a>`
/* FP:hir.rs-0063 */     PreciseCapturing,
/* FP:hir.rs-0064 */ 
/* FP:hir.rs-0065 */     /// Other usages which have not yet been categorized. Feel free to
/* FP:hir.rs-0066 */     /// add new sources that you find useful.
/* FP:hir.rs-0067 */     ///
/* FP:hir.rs-0068 */     /// Some non-exhaustive examples:
/* FP:hir.rs-0069 */     /// - `where T: 'a`
/* FP:hir.rs-0070 */     /// - `fn(_: dyn Trait + 'a)`
/* FP:hir.rs-0071 */     Other,
/* FP:hir.rs-0072 */ }
/* FP:hir.rs-0073 */ 
/* FP:hir.rs-0074 */ #[derive(Debug, Copy, Clone, PartialEq, Eq, HashStable_Generic)]
/* FP:hir.rs-0075 */ pub enum LifetimeSyntax {
/* FP:hir.rs-0076 */     /// E.g. `&Type`, `ContainsLifetime`
/* FP:hir.rs-0077 */     Implicit,
/* FP:hir.rs-0078 */ 
/* FP:hir.rs-0079 */     /// E.g. `&'_ Type`, `ContainsLifetime<'_>`, `impl Trait + '_`, `impl Trait + use<'_>`
/* FP:hir.rs-0080 */     ExplicitAnonymous,
/* FP:hir.rs-0081 */ 
/* FP:hir.rs-0082 */     /// E.g. `&'a Type`, `ContainsLifetime<'a>`, `impl Trait + 'a`, `impl Trait + use<'a>`
/* FP:hir.rs-0083 */     ExplicitBound,
/* FP:hir.rs-0084 */ }
/* FP:hir.rs-0085 */ 
/* FP:hir.rs-0086 */ impl From<Ident> for LifetimeSyntax {
/* FP:hir.rs-0087 */     fn from(ident: Ident) -> Self {
/* FP:hir.rs-0088 */         let name = ident.name;
/* FP:hir.rs-0089 */ 
/* FP:hir.rs-0090 */         if name == sym::empty {
/* FP:hir.rs-0091 */             unreachable!("A lifetime name should never be empty");
/* FP:hir.rs-0092 */         } else if name == kw::UnderscoreLifetime {
/* FP:hir.rs-0093 */             LifetimeSyntax::ExplicitAnonymous
/* FP:hir.rs-0094 */         } else {
/* FP:hir.rs-0095 */             debug_assert!(name.as_str().starts_with('\''));
/* FP:hir.rs-0096 */             LifetimeSyntax::ExplicitBound
/* FP:hir.rs-0097 */         }
/* FP:hir.rs-0098 */     }
/* FP:hir.rs-0099 */ }
/* FP:hir.rs-0100 */ 
/* FP:hir.rs-0101 */ /// A lifetime. The valid field combinations are non-obvious and not all
/* FP:hir.rs-0102 */ /// combinations are possible. The following example shows some of
/* FP:hir.rs-0103 */ /// them. See also the comments on `LifetimeKind` and `LifetimeSource`.
/* FP:hir.rs-0104 */ ///
/* FP:hir.rs-0105 */ /// ```
/* FP:hir.rs-0106 */ /// #[repr(C)]
/* FP:hir.rs-0107 */ /// struct S<'a>(&'a u32);       // res=Param, name='a, source=Reference, syntax=ExplicitBound
/* FP:hir.rs-0108 */ /// unsafe extern "C" {
/* FP:hir.rs-0109 */ ///     fn f1(s: S);             // res=Param, name='_, source=Path, syntax=Implicit
/* FP:hir.rs-0110 */ ///     fn f2(s: S<'_>);         // res=Param, name='_, source=Path, syntax=ExplicitAnonymous
/* FP:hir.rs-0111 */ ///     fn f3<'a>(s: S<'a>);     // res=Param, name='a, source=Path, syntax=ExplicitBound
/* FP:hir.rs-0112 */ /// }
/* FP:hir.rs-0113 */ ///
/* FP:hir.rs-0114 */ /// struct St<'a> { x: &'a u32 } // res=Param, name='a, source=Reference, syntax=ExplicitBound
/* FP:hir.rs-0115 */ /// fn f() {
/* FP:hir.rs-0116 */ ///     _ = St { x: &0 };        // res=Infer, name='_, source=Path, syntax=Implicit
/* FP:hir.rs-0117 */ ///     _ = St::<'_> { x: &0 };  // res=Infer, name='_, source=Path, syntax=ExplicitAnonymous
/* FP:hir.rs-0118 */ /// }
/* FP:hir.rs-0119 */ ///
/* FP:hir.rs-0120 */ /// struct Name<'a>(&'a str);    // res=Param,  name='a, source=Reference, syntax=ExplicitBound
/* FP:hir.rs-0121 */ /// const A: Name = Name("a");   // res=Static, name='_, source=Path, syntax=Implicit
/* FP:hir.rs-0122 */ /// const B: &str = "";          // res=Static, name='_, source=Reference, syntax=Implicit
/* FP:hir.rs-0123 */ /// static C: &'_ str = "";      // res=Static, name='_, source=Reference, syntax=ExplicitAnonymous
/* FP:hir.rs-0124 */ /// static D: &'static str = ""; // res=Static, name='static, source=Reference, syntax=ExplicitBound
/* FP:hir.rs-0125 */ ///
/* FP:hir.rs-0126 */ /// trait Tr {}
/* FP:hir.rs-0127 */ /// fn tr(_: Box<dyn Tr>) {}     // res=ImplicitObjectLifetimeDefault, name='_, source=Other, syntax=Implicit
/* FP:hir.rs-0128 */ ///
/* FP:hir.rs-0129 */ /// fn capture_outlives<'a>() ->
/* FP:hir.rs-0130 */ ///     impl FnOnce() + 'a       // res=Param, ident='a, source=OutlivesBound, syntax=ExplicitBound
/* FP:hir.rs-0131 */ /// {
/* FP:hir.rs-0132 */ ///     || {}
/* FP:hir.rs-0133 */ /// }
/* FP:hir.rs-0134 */ ///
/* FP:hir.rs-0135 */ /// fn capture_precise<'a>() ->
/* FP:hir.rs-0136 */ ///     impl FnOnce() + use<'a>  // res=Param, ident='a, source=PreciseCapturing, syntax=ExplicitBound
/* FP:hir.rs-0137 */ /// {
/* FP:hir.rs-0138 */ ///     || {}
/* FP:hir.rs-0139 */ /// }
/* FP:hir.rs-0140 */ ///
/* FP:hir.rs-0141 */ /// // (commented out because these cases trigger errors)
/* FP:hir.rs-0142 */ /// // struct S1<'a>(&'a str);   // res=Param, name='a, source=Reference, syntax=ExplicitBound
/* FP:hir.rs-0143 */ /// // struct S2(S1);            // res=Error, name='_, source=Path, syntax=Implicit
/* FP:hir.rs-0144 */ /// // struct S3(S1<'_>);        // res=Error, name='_, source=Path, syntax=ExplicitAnonymous
/* FP:hir.rs-0145 */ /// // struct S4(S1<'a>);        // res=Error, name='a, source=Path, syntax=ExplicitBound
/* FP:hir.rs-0146 */ /// ```
/* FP:hir.rs-0147 */ ///
/* FP:hir.rs-0148 */ /// Some combinations that cannot occur are `LifetimeSyntax::Implicit` with
/* FP:hir.rs-0149 */ /// `LifetimeSource::OutlivesBound` or `LifetimeSource::PreciseCapturing`
/* FP:hir.rs-0150 */ /// — there's no way to "elide" these lifetimes.
/* FP:hir.rs-0151 */ #[derive(Debug, Copy, Clone, HashStable_Generic)]
/* FP:hir.rs-0152 */ // Raise the aligement to at least 4 bytes - this is relied on in other parts of the compiler(for pointer tagging):
/* FP:hir.rs-0153 */ // https://github.com/rust-lang/rust/blob/ce5fdd7d42aba9a2925692e11af2bd39cf37798a/compiler/rustc_data_structures/src/tagged_ptr.rs#L163
/* FP:hir.rs-0154 */ // Removing this `repr(4)` will cause the compiler to not build on platforms like `m68k` Linux, where the aligement of u32 and usize is only 2.
/* FP:hir.rs-0155 */ // Since `repr(align)` may only raise aligement, this has no effect on platforms where the aligement is already sufficient.
/* FP:hir.rs-0156 */ #[repr(align(4))]
/* FP:hir.rs-0157 */ pub struct Lifetime {
/* FP:hir.rs-0158 */     #[stable_hasher(ignore)]
/* FP:hir.rs-0159 */     pub hir_id: HirId,
/* FP:hir.rs-0160 */ 
/* FP:hir.rs-0161 */     /// Either a named lifetime definition (e.g. `'a`, `'static`) or an
/* FP:hir.rs-0162 */     /// anonymous lifetime (`'_`, either explicitly written, or inserted for
/* FP:hir.rs-0163 */     /// things like `&type`).
/* FP:hir.rs-0164 */     pub ident: Ident,
/* FP:hir.rs-0165 */ 
/* FP:hir.rs-0166 */     /// Semantics of this lifetime.
/* FP:hir.rs-0167 */     pub kind: LifetimeKind,
/* FP:hir.rs-0168 */ 
/* FP:hir.rs-0169 */     /// The context in which the lifetime occurred. See `Lifetime::suggestion`
/* FP:hir.rs-0170 */     /// for example use.
/* FP:hir.rs-0171 */     pub source: LifetimeSource,
/* FP:hir.rs-0172 */ 
/* FP:hir.rs-0173 */     /// The syntax that the user used to declare this lifetime. See
/* FP:hir.rs-0174 */     /// `Lifetime::suggestion` for example use.
/* FP:hir.rs-0175 */     pub syntax: LifetimeSyntax,
/* FP:hir.rs-0176 */ }
/* FP:hir.rs-0177 */ 
/* FP:hir.rs-0178 */ #[derive(Debug, Copy, Clone, HashStable_Generic)]
/* FP:hir.rs-0179 */ pub enum ParamName {
/* FP:hir.rs-0180 */     /// Some user-given name like `T` or `'x`.
/* FP:hir.rs-0181 */     Plain(Ident),
/* FP:hir.rs-0182 */ 
/* FP:hir.rs-0183 */     /// Indicates an illegal name was given and an error has been
/* FP:hir.rs-0184 */     /// reported (so we should squelch other derived errors).
/* FP:hir.rs-0185 */     ///
/* FP:hir.rs-0186 */     /// Occurs when, e.g., `'_` is used in the wrong place, or a
/* FP:hir.rs-0187 */     /// lifetime name is duplicated.
/* FP:hir.rs-0188 */     Error(Ident),
/* FP:hir.rs-0189 */ 
/* FP:hir.rs-0190 */     /// Synthetic name generated when user elided a lifetime in an impl header.
/* FP:hir.rs-0191 */     ///
/* FP:hir.rs-0192 */     /// E.g., the lifetimes in cases like these:
/* FP:hir.rs-0193 */     /// ```ignore (fragment)
/* FP:hir.rs-0194 */     /// impl Foo for &u32
/* FP:hir.rs-0195 */     /// impl Foo<'_> for u32
/* FP:hir.rs-0196 */     /// ```
/* FP:hir.rs-0197 */     /// in that case, we rewrite to
/* FP:hir.rs-0198 */     /// ```ignore (fragment)
/* FP:hir.rs-0199 */     /// impl<'f> Foo for &'f u32
/* FP:hir.rs-0200 */     /// impl<'f> Foo<'f> for u32
/* FP:hir.rs-0201 */     /// ```
/* FP:hir.rs-0202 */     /// where `'f` is something like `Fresh(0)`. The indices are
/* FP:hir.rs-0203 */     /// unique per impl, but not necessarily continuous.
/* FP:hir.rs-0204 */     Fresh,
/* FP:hir.rs-0205 */ }
/* FP:hir.rs-0206 */ 
/* FP:hir.rs-0207 */ impl ParamName {
/* FP:hir.rs-0208 */     pub fn ident(&self) -> Ident {
/* FP:hir.rs-0209 */         match *self {
/* FP:hir.rs-0210 */             ParamName::Plain(ident) | ParamName::Error(ident) => ident,
/* FP:hir.rs-0211 */             ParamName::Fresh => Ident::with_dummy_span(kw::UnderscoreLifetime),
/* FP:hir.rs-0212 */         }
/* FP:hir.rs-0213 */     }
/* FP:hir.rs-0214 */ }
/* FP:hir.rs-0215 */ 
/* FP:hir.rs-0216 */ #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, HashStable_Generic)]
/* FP:hir.rs-0217 */ pub enum LifetimeKind {
/* FP:hir.rs-0218 */     /// User-given names or fresh (synthetic) names.
/* FP:hir.rs-0219 */     Param(LocalDefId),
/* FP:hir.rs-0220 */ 
/* FP:hir.rs-0221 */     /// Implicit lifetime in a context like `dyn Foo`. This is
/* FP:hir.rs-0222 */     /// distinguished from implicit lifetimes elsewhere because the
/* FP:hir.rs-0223 */     /// lifetime that they default to must appear elsewhere within the
/* FP:hir.rs-0224 */     /// enclosing type. This means that, in an `impl Trait` context, we
/* FP:hir.rs-0225 */     /// don't have to create a parameter for them. That is, `impl
/* FP:hir.rs-0226 */     /// Trait<Item = &u32>` expands to an opaque type like `type
/* FP:hir.rs-0227 */     /// Foo<'a> = impl Trait<Item = &'a u32>`, but `impl Trait<item =
/* FP:hir.rs-0228 */     /// dyn Bar>` expands to `type Foo = impl Trait<Item = dyn Bar +
/* FP:hir.rs-0229 */     /// 'static>`. The latter uses `ImplicitObjectLifetimeDefault` so
/* FP:hir.rs-0230 */     /// that surrounding code knows not to create a lifetime
/* FP:hir.rs-0231 */     /// parameter.
/* FP:hir.rs-0232 */     ImplicitObjectLifetimeDefault,
/* FP:hir.rs-0233 */ 
/* FP:hir.rs-0234 */     /// Indicates an error during lowering (usually `'_` in wrong place)
/* FP:hir.rs-0235 */     /// that was already reported.
/* FP:hir.rs-0236 */     Error,
/* FP:hir.rs-0237 */ 
/* FP:hir.rs-0238 */     /// User wrote an anonymous lifetime, either `'_` or nothing (which gets
/* FP:hir.rs-0239 */     /// converted to `'_`). The semantics of this lifetime should be inferred
/* FP:hir.rs-0240 */     /// by typechecking code.
/* FP:hir.rs-0241 */     Infer,
/* FP:hir.rs-0242 */ 
/* FP:hir.rs-0243 */     /// User wrote `'static` or nothing (which gets converted to `'_`).
/* FP:hir.rs-0244 */     Static,
/* FP:hir.rs-0245 */ }
/* FP:hir.rs-0246 */ 
/* FP:hir.rs-0247 */ impl LifetimeKind {
/* FP:hir.rs-0248 */     fn is_elided(&self) -> bool {
/* FP:hir.rs-0249 */         match self {
/* FP:hir.rs-0250 */             LifetimeKind::ImplicitObjectLifetimeDefault | LifetimeKind::Infer => true,
/* FP:hir.rs-0251 */ 
/* FP:hir.rs-0252 */             // It might seem surprising that `Fresh` counts as not *elided*
/* FP:hir.rs-0253 */             // -- but this is because, as far as the code in the compiler is
/* FP:hir.rs-0254 */             // concerned -- `Fresh` variants act equivalently to "some fresh name".
/* FP:hir.rs-0255 */             // They correspond to early-bound regions on an impl, in other words.
/* FP:hir.rs-0256 */             LifetimeKind::Error | LifetimeKind::Param(..) | LifetimeKind::Static => false,
/* FP:hir.rs-0257 */         }
/* FP:hir.rs-0258 */     }
/* FP:hir.rs-0259 */ }
/* FP:hir.rs-0260 */ 
/* FP:hir.rs-0261 */ impl fmt::Display for Lifetime {
/* FP:hir.rs-0262 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-0263 */         self.ident.name.fmt(f)
/* FP:hir.rs-0264 */     }
/* FP:hir.rs-0265 */ }
/* FP:hir.rs-0266 */ 
/* FP:hir.rs-0267 */ impl Lifetime {
/* FP:hir.rs-0268 */     pub fn new(
/* FP:hir.rs-0269 */         hir_id: HirId,
/* FP:hir.rs-0270 */         ident: Ident,
/* FP:hir.rs-0271 */         kind: LifetimeKind,
/* FP:hir.rs-0272 */         source: LifetimeSource,
/* FP:hir.rs-0273 */         syntax: LifetimeSyntax,
/* FP:hir.rs-0274 */     ) -> Lifetime {
/* FP:hir.rs-0275 */         let lifetime = Lifetime { hir_id, ident, kind, source, syntax };
/* FP:hir.rs-0276 */ 
/* FP:hir.rs-0277 */         // Sanity check: elided lifetimes form a strict subset of anonymous lifetimes.
/* FP:hir.rs-0278 */         #[cfg(debug_assertions)]
/* FP:hir.rs-0279 */         match (lifetime.is_elided(), lifetime.is_anonymous()) {
/* FP:hir.rs-0280 */             (false, false) => {} // e.g. `'a`
/* FP:hir.rs-0281 */             (false, true) => {}  // e.g. explicit `'_`
/* FP:hir.rs-0282 */             (true, true) => {}   // e.g. `&x`
/* FP:hir.rs-0283 */             (true, false) => panic!("bad Lifetime"),
/* FP:hir.rs-0284 */         }
/* FP:hir.rs-0285 */ 
/* FP:hir.rs-0286 */         lifetime
/* FP:hir.rs-0287 */     }
/* FP:hir.rs-0288 */ 
/* FP:hir.rs-0289 */     pub fn is_elided(&self) -> bool {
/* FP:hir.rs-0290 */         self.kind.is_elided()
/* FP:hir.rs-0291 */     }
/* FP:hir.rs-0292 */ 
/* FP:hir.rs-0293 */     pub fn is_anonymous(&self) -> bool {
/* FP:hir.rs-0294 */         self.ident.name == kw::UnderscoreLifetime
/* FP:hir.rs-0295 */     }
/* FP:hir.rs-0296 */ 
/* FP:hir.rs-0297 */     pub fn is_implicit(&self) -> bool {
/* FP:hir.rs-0298 */         matches!(self.syntax, LifetimeSyntax::Implicit)
/* FP:hir.rs-0299 */     }
/* FP:hir.rs-0300 */ 
/* FP:hir.rs-0301 */     pub fn is_static(&self) -> bool {
/* FP:hir.rs-0302 */         self.kind == LifetimeKind::Static
/* FP:hir.rs-0303 */     }
/* FP:hir.rs-0304 */ 
/* FP:hir.rs-0305 */     pub fn suggestion(&self, new_lifetime: &str) -> (Span, String) {
/* FP:hir.rs-0306 */         use LifetimeSource::*;
/* FP:hir.rs-0307 */         use LifetimeSyntax::*;
/* FP:hir.rs-0308 */ 
/* FP:hir.rs-0309 */         debug_assert!(new_lifetime.starts_with('\''));
/* FP:hir.rs-0310 */ 
/* FP:hir.rs-0311 */         match (self.syntax, self.source) {
/* FP:hir.rs-0312 */             // The user wrote `'a` or `'_`.
/* FP:hir.rs-0313 */             (ExplicitBound | ExplicitAnonymous, _) => (self.ident.span, format!("{new_lifetime}")),
/* FP:hir.rs-0314 */ 
/* FP:hir.rs-0315 */             // The user wrote `Path<T>`, and omitted the `'_,`.
/* FP:hir.rs-0316 */             (Implicit, Path { angle_brackets: AngleBrackets::Full }) => {
/* FP:hir.rs-0317 */                 (self.ident.span, format!("{new_lifetime}, "))
/* FP:hir.rs-0318 */             }
/* FP:hir.rs-0319 */ 
/* FP:hir.rs-0320 */             // The user wrote `Path<>`, and omitted the `'_`..
/* FP:hir.rs-0321 */             (Implicit, Path { angle_brackets: AngleBrackets::Empty }) => {
/* FP:hir.rs-0322 */                 (self.ident.span, format!("{new_lifetime}"))
/* FP:hir.rs-0323 */             }
/* FP:hir.rs-0324 */ 
/* FP:hir.rs-0325 */             // The user wrote `Path` and omitted the `<'_>`.
/* FP:hir.rs-0326 */             (Implicit, Path { angle_brackets: AngleBrackets::Missing }) => {
/* FP:hir.rs-0327 */                 (self.ident.span.shrink_to_hi(), format!("<{new_lifetime}>"))
/* FP:hir.rs-0328 */             }
/* FP:hir.rs-0329 */ 
/* FP:hir.rs-0330 */             // The user wrote `&type` or `&mut type`.
/* FP:hir.rs-0331 */             (Implicit, Reference) => (self.ident.span, format!("{new_lifetime} ")),
/* FP:hir.rs-0332 */ 
/* FP:hir.rs-0333 */             (Implicit, source) => {
/* FP:hir.rs-0334 */                 unreachable!("can't suggest for a implicit lifetime of {source:?}")
/* FP:hir.rs-0335 */             }
/* FP:hir.rs-0336 */         }
/* FP:hir.rs-0337 */     }
/* FP:hir.rs-0338 */ }
/* FP:hir.rs-0339 */ 
/* FP:hir.rs-0340 */ /// A `Path` is essentially Rust's notion of a name; for instance,
/* FP:hir.rs-0341 */ /// `std::cmp::PartialEq`. It's represented as a sequence of identifiers,
/* FP:hir.rs-0342 */ /// along with a bunch of supporting information.
/* FP:hir.rs-0343 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-0344 */ pub struct Path<'hir, R = Res> {
/* FP:hir.rs-0345 */     pub span: Span,
/* FP:hir.rs-0346 */     /// The resolution for the path.
/* FP:hir.rs-0347 */     pub res: R,
/* FP:hir.rs-0348 */     /// The segments in the path: the things separated by `::`.
/* FP:hir.rs-0349 */     pub segments: &'hir [PathSegment<'hir>],
/* FP:hir.rs-0350 */ }
/* FP:hir.rs-0351 */ 
/* FP:hir.rs-0352 */ /// Up to three resolutions for type, value and macro namespaces.
/* FP:hir.rs-0353 */ pub type UsePath<'hir> = Path<'hir, PerNS<Option<Res>>>;
/* FP:hir.rs-0354 */ 
/* FP:hir.rs-0355 */ impl Path<'_> {
/* FP:hir.rs-0356 */     pub fn is_global(&self) -> bool {
/* FP:hir.rs-0357 */         self.segments.first().is_some_and(|segment| segment.ident.name == kw::PathRoot)
/* FP:hir.rs-0358 */     }
/* FP:hir.rs-0359 */ }
/* FP:hir.rs-0360 */ 
/* FP:hir.rs-0361 */ /// A segment of a path: an identifier, an optional lifetime, and a set of
/* FP:hir.rs-0362 */ /// types.
/* FP:hir.rs-0363 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-0364 */ pub struct PathSegment<'hir> {
/* FP:hir.rs-0365 */     /// The identifier portion of this path segment.
/* FP:hir.rs-0366 */     pub ident: Ident,
/* FP:hir.rs-0367 */     #[stable_hasher(ignore)]
/* FP:hir.rs-0368 */     pub hir_id: HirId,
/* FP:hir.rs-0369 */     pub res: Res,
/* FP:hir.rs-0370 */ 
/* FP:hir.rs-0371 */     /// Type/lifetime parameters attached to this path. They come in
/* FP:hir.rs-0372 */     /// two flavors: `Path<A,B,C>` and `Path(A,B) -> C`. Note that
/* FP:hir.rs-0373 */     /// this is more than just simple syntactic sugar; the use of
/* FP:hir.rs-0374 */     /// parens affects the region binding rules, so we preserve the
/* FP:hir.rs-0375 */     /// distinction.
/* FP:hir.rs-0376 */     pub args: Option<&'hir GenericArgs<'hir>>,
/* FP:hir.rs-0377 */ 
/* FP:hir.rs-0378 */     /// Whether to infer remaining type parameters, if any.
/* FP:hir.rs-0379 */     /// This only applies to expression and pattern paths, and
/* FP:hir.rs-0380 */     /// out of those only the segments with no type parameters
/* FP:hir.rs-0381 */     /// to begin with, e.g., `Vec::new` is `<Vec<..>>::new::<..>`.
/* FP:hir.rs-0382 */     pub infer_args: bool,
/* FP:hir.rs-0383 */ }
/* FP:hir.rs-0384 */ 
/* FP:hir.rs-0385 */ impl<'hir> PathSegment<'hir> {
/* FP:hir.rs-0386 */     /// Converts an identifier to the corresponding segment.
/* FP:hir.rs-0387 */     pub fn new(ident: Ident, hir_id: HirId, res: Res) -> PathSegment<'hir> {
/* FP:hir.rs-0388 */         PathSegment { ident, hir_id, res, infer_args: true, args: None }
/* FP:hir.rs-0389 */     }
/* FP:hir.rs-0390 */ 
/* FP:hir.rs-0391 */     pub fn invalid() -> Self {
/* FP:hir.rs-0392 */         Self::new(Ident::dummy(), HirId::INVALID, Res::Err)
/* FP:hir.rs-0393 */     }
/* FP:hir.rs-0394 */ 
/* FP:hir.rs-0395 */     pub fn args(&self) -> &GenericArgs<'hir> {
/* FP:hir.rs-0396 */         if let Some(ref args) = self.args {
/* FP:hir.rs-0397 */             args
/* FP:hir.rs-0398 */         } else {
/* FP:hir.rs-0399 */             const DUMMY: &GenericArgs<'_> = &GenericArgs::none();
/* FP:hir.rs-0400 */             DUMMY
/* FP:hir.rs-0401 */         }
/* FP:hir.rs-0402 */     }
/* FP:hir.rs-0403 */ }
/* FP:hir.rs-0404 */ 
/* FP:hir.rs-0405 */ /// A constant that enters the type system, used for arguments to const generics (e.g. array lengths).
/* FP:hir.rs-0406 */ ///
/* FP:hir.rs-0407 */ /// These are distinct from [`AnonConst`] as anon consts in the type system are not allowed
/* FP:hir.rs-0408 */ /// to use any generic parameters, therefore we must represent `N` differently. Additionally
/* FP:hir.rs-0409 */ /// future designs for supporting generic parameters in const arguments will likely not use
/* FP:hir.rs-0410 */ /// an anon const based design.
/* FP:hir.rs-0411 */ ///
/* FP:hir.rs-0412 */ /// So, `ConstArg` (specifically, [`ConstArgKind`]) distinguishes between const args
/* FP:hir.rs-0413 */ /// that are [just paths](ConstArgKind::Path) (currently just bare const params)
/* FP:hir.rs-0414 */ /// versus const args that are literals or have arbitrary computations (e.g., `{ 1 + 3 }`).
/* FP:hir.rs-0415 */ ///
/* FP:hir.rs-0416 */ /// For an explanation of the `Unambig` generic parameter see the dev-guide:
/* FP:hir.rs-0417 */ /// <https://rustc-dev-guide.rust-lang.org/hir/ambig-unambig-ty-and-consts.html>
/* FP:hir.rs-0418 */ #[derive(Clone, Copy, Debug, HashStable_Generic)]
/* FP:hir.rs-0419 */ #[repr(C)]
/* FP:hir.rs-0420 */ pub struct ConstArg<'hir, Unambig = ()> {
/* FP:hir.rs-0421 */     #[stable_hasher(ignore)]
/* FP:hir.rs-0422 */     pub hir_id: HirId,
/* FP:hir.rs-0423 */     pub kind: ConstArgKind<'hir, Unambig>,
/* FP:hir.rs-0424 */ }
/* FP:hir.rs-0425 */ 
/* FP:hir.rs-0426 */ impl<'hir> ConstArg<'hir, AmbigArg> {
/* FP:hir.rs-0427 */     /// Converts a `ConstArg` in an ambiguous position to one in an unambiguous position.
/* FP:hir.rs-0428 */     ///
/* FP:hir.rs-0429 */     /// Functions accepting unambiguous consts may expect the [`ConstArgKind::Infer`] variant
/* FP:hir.rs-0430 */     /// to be used. Care should be taken to separately handle infer consts when calling this
/* FP:hir.rs-0431 */     /// function as it cannot be handled by downstream code making use of the returned const.
/* FP:hir.rs-0432 */     ///
/* FP:hir.rs-0433 */     /// In practice this may mean overriding the [`Visitor::visit_infer`][visit_infer] method on hir visitors, or
/* FP:hir.rs-0434 */     /// specifically matching on [`GenericArg::Infer`] when handling generic arguments.
/* FP:hir.rs-0435 */     ///
/* FP:hir.rs-0436 */     /// [visit_infer]: [crate::rustc_hir::intravisit::Visitor::visit_infer]
/* FP:hir.rs-0437 */     pub fn as_unambig_ct(&self) -> &ConstArg<'hir> {
/* FP:hir.rs-0438 */         // SAFETY: `ConstArg` is `repr(C)` and `ConstArgKind` is marked `repr(u8)` so that the
/* FP:hir.rs-0439 */         // layout is the same across different ZST type arguments.
/* FP:hir.rs-0440 */         let ptr = self as *const ConstArg<'hir, AmbigArg> as *const ConstArg<'hir, ()>;
/* FP:hir.rs-0441 */         unsafe { &*ptr }
/* FP:hir.rs-0442 */     }
/* FP:hir.rs-0443 */ }
/* FP:hir.rs-0444 */ 
/* FP:hir.rs-0445 */ impl<'hir> ConstArg<'hir> {
/* FP:hir.rs-0446 */     /// Converts a `ConstArg` in an unambiguous position to one in an ambiguous position. This is
/* FP:hir.rs-0447 */     /// fallible as the [`ConstArgKind::Infer`] variant is not present in ambiguous positions.
/* FP:hir.rs-0448 */     ///
/* FP:hir.rs-0449 */     /// Functions accepting ambiguous consts will not handle the [`ConstArgKind::Infer`] variant, if
/* FP:hir.rs-0450 */     /// infer consts are relevant to you then care should be taken to handle them separately.
/* FP:hir.rs-0451 */     pub fn try_as_ambig_ct(&self) -> Option<&ConstArg<'hir, AmbigArg>> {
/* FP:hir.rs-0452 */         if let ConstArgKind::Infer(_, ()) = self.kind {
/* FP:hir.rs-0453 */             return None;
/* FP:hir.rs-0454 */         }
/* FP:hir.rs-0455 */ 
/* FP:hir.rs-0456 */         // SAFETY: `ConstArg` is `repr(C)` and `ConstArgKind` is marked `repr(u8)` so that the layout is
/* FP:hir.rs-0457 */         // the same across different ZST type arguments. We also asserted that the `self` is
/* FP:hir.rs-0458 */         // not a `ConstArgKind::Infer` so there is no risk of transmuting a `()` to `AmbigArg`.
/* FP:hir.rs-0459 */         let ptr = self as *const ConstArg<'hir> as *const ConstArg<'hir, AmbigArg>;
/* FP:hir.rs-0460 */         Some(unsafe { &*ptr })
/* FP:hir.rs-0461 */     }
/* FP:hir.rs-0462 */ }
/* FP:hir.rs-0463 */ 
/* FP:hir.rs-0464 */ impl<'hir, Unambig> ConstArg<'hir, Unambig> {
/* FP:hir.rs-0465 */     pub fn anon_const_hir_id(&self) -> Option<HirId> {
/* FP:hir.rs-0466 */         match self.kind {
/* FP:hir.rs-0467 */             ConstArgKind::Anon(ac) => Some(ac.hir_id),
/* FP:hir.rs-0468 */             _ => None,
/* FP:hir.rs-0469 */         }
/* FP:hir.rs-0470 */     }
/* FP:hir.rs-0471 */ 
/* FP:hir.rs-0472 */     pub fn span(&self) -> Span {
/* FP:hir.rs-0473 */         match self.kind {
/* FP:hir.rs-0474 */             ConstArgKind::Path(path) => path.span(),
/* FP:hir.rs-0475 */             ConstArgKind::Anon(anon) => anon.span,
/* FP:hir.rs-0476 */             ConstArgKind::Infer(span, _) => span,
/* FP:hir.rs-0477 */         }
/* FP:hir.rs-0478 */     }
/* FP:hir.rs-0479 */ }
/* FP:hir.rs-0480 */ 
/* FP:hir.rs-0481 */ /// See [`ConstArg`].
/* FP:hir.rs-0482 */ #[derive(Clone, Copy, Debug, HashStable_Generic)]
/* FP:hir.rs-0483 */ #[repr(u8, C)]
/* FP:hir.rs-0484 */ pub enum ConstArgKind<'hir, Unambig = ()> {
/* FP:hir.rs-0485 */     /// **Note:** Currently this is only used for bare const params
/* FP:hir.rs-0486 */     /// (`N` where `fn foo<const N: usize>(...)`),
/* FP:hir.rs-0487 */     /// not paths to any const (`N` where `const N: usize = ...`).
/* FP:hir.rs-0488 */     ///
/* FP:hir.rs-0489 */     /// However, in the future, we'll be using it for all of those.
/* FP:hir.rs-0490 */     Path(QPath<'hir>),
/* FP:hir.rs-0491 */     Anon(&'hir AnonConst),
/* FP:hir.rs-0492 */     /// This variant is not always used to represent inference consts, sometimes
/* FP:hir.rs-0493 */     /// [`GenericArg::Infer`] is used instead.
/* FP:hir.rs-0494 */     Infer(Span, Unambig),
/* FP:hir.rs-0495 */ }
/* FP:hir.rs-0496 */ 
/* FP:hir.rs-0497 */ #[derive(Clone, Copy, Debug, HashStable_Generic)]
/* FP:hir.rs-0498 */ pub struct InferArg {
/* FP:hir.rs-0499 */     #[stable_hasher(ignore)]
/* FP:hir.rs-0500 */     pub hir_id: HirId,
/* FP:hir.rs-0501 */     pub span: Span,
/* FP:hir.rs-0502 */ }
/* FP:hir.rs-0503 */ 
/* FP:hir.rs-0504 */ impl InferArg {
/* FP:hir.rs-0505 */     pub fn to_ty(&self) -> Ty<'static> {
/* FP:hir.rs-0506 */         Ty { kind: TyKind::Infer(()), span: self.span, hir_id: self.hir_id }
/* FP:hir.rs-0507 */     }
/* FP:hir.rs-0508 */ }
/* FP:hir.rs-0509 */ 
/* FP:hir.rs-0510 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-0511 */ pub enum GenericArg<'hir> {
/* FP:hir.rs-0512 */     Lifetime(&'hir Lifetime),
/* FP:hir.rs-0513 */     Type(&'hir Ty<'hir, AmbigArg>),
/* FP:hir.rs-0514 */     Const(&'hir ConstArg<'hir, AmbigArg>),
/* FP:hir.rs-0515 */     /// Inference variables in [`GenericArg`] are always represented by
/* FP:hir.rs-0516 */     /// `GenericArg::Infer` instead of the `Infer` variants on [`TyKind`] and
/* FP:hir.rs-0517 */     /// [`ConstArgKind`] as it is not clear until hir ty lowering whether a
/* FP:hir.rs-0518 */     /// `_` argument is a type or const argument.
/* FP:hir.rs-0519 */     ///
/* FP:hir.rs-0520 */     /// However, some builtin types' generic arguments are represented by [`TyKind`]
/* FP:hir.rs-0521 */     /// without a [`GenericArg`], instead directly storing a [`Ty`] or [`ConstArg`]. In
/* FP:hir.rs-0522 */     /// such cases they *are* represented by the `Infer` variants on [`TyKind`] and
/* FP:hir.rs-0523 */     /// [`ConstArgKind`] as it is not ambiguous whether the argument is a type or const.
/* FP:hir.rs-0524 */     Infer(InferArg),
/* FP:hir.rs-0525 */ }
/* FP:hir.rs-0526 */ 
/* FP:hir.rs-0527 */ impl GenericArg<'_> {
/* FP:hir.rs-0528 */     pub fn span(&self) -> Span {
/* FP:hir.rs-0529 */         match self {
/* FP:hir.rs-0530 */             GenericArg::Lifetime(l) => l.ident.span,
/* FP:hir.rs-0531 */             GenericArg::Type(t) => t.span,
/* FP:hir.rs-0532 */             GenericArg::Const(c) => c.span(),
/* FP:hir.rs-0533 */             GenericArg::Infer(i) => i.span,
/* FP:hir.rs-0534 */         }
/* FP:hir.rs-0535 */     }
/* FP:hir.rs-0536 */ 
/* FP:hir.rs-0537 */     pub fn hir_id(&self) -> HirId {
/* FP:hir.rs-0538 */         match self {
/* FP:hir.rs-0539 */             GenericArg::Lifetime(l) => l.hir_id,
/* FP:hir.rs-0540 */             GenericArg::Type(t) => t.hir_id,
/* FP:hir.rs-0541 */             GenericArg::Const(c) => c.hir_id,
/* FP:hir.rs-0542 */             GenericArg::Infer(i) => i.hir_id,
/* FP:hir.rs-0543 */         }
/* FP:hir.rs-0544 */     }
/* FP:hir.rs-0545 */ 
/* FP:hir.rs-0546 */     pub fn descr(&self) -> &'static str {
/* FP:hir.rs-0547 */         match self {
/* FP:hir.rs-0548 */             GenericArg::Lifetime(_) => "lifetime",
/* FP:hir.rs-0549 */             GenericArg::Type(_) => "type",
/* FP:hir.rs-0550 */             GenericArg::Const(_) => "constant",
/* FP:hir.rs-0551 */             GenericArg::Infer(_) => "placeholder",
/* FP:hir.rs-0552 */         }
/* FP:hir.rs-0553 */     }
/* FP:hir.rs-0554 */ 
/* FP:hir.rs-0555 */     pub fn to_ord(&self) -> ast::ParamKindOrd {
/* FP:hir.rs-0556 */         match self {
/* FP:hir.rs-0557 */             GenericArg::Lifetime(_) => ast::ParamKindOrd::Lifetime,
/* FP:hir.rs-0558 */             GenericArg::Type(_) | GenericArg::Const(_) | GenericArg::Infer(_) => {
/* FP:hir.rs-0559 */                 ast::ParamKindOrd::TypeOrConst
/* FP:hir.rs-0560 */             }
/* FP:hir.rs-0561 */         }
/* FP:hir.rs-0562 */     }
/* FP:hir.rs-0563 */ 
/* FP:hir.rs-0564 */     pub fn is_ty_or_const(&self) -> bool {
/* FP:hir.rs-0565 */         match self {
/* FP:hir.rs-0566 */             GenericArg::Lifetime(_) => false,
/* FP:hir.rs-0567 */             GenericArg::Type(_) | GenericArg::Const(_) | GenericArg::Infer(_) => true,
/* FP:hir.rs-0568 */         }
/* FP:hir.rs-0569 */     }
/* FP:hir.rs-0570 */ }
/* FP:hir.rs-0571 */ 
/* FP:hir.rs-0572 */ /// The generic arguments and associated item constraints of a path segment.
/* FP:hir.rs-0573 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-0574 */ pub struct GenericArgs<'hir> {
/* FP:hir.rs-0575 */     /// The generic arguments for this path segment.
/* FP:hir.rs-0576 */     pub args: &'hir [GenericArg<'hir>],
/* FP:hir.rs-0577 */     /// The associated item constraints for this path segment.
/* FP:hir.rs-0578 */     pub constraints: &'hir [AssocItemConstraint<'hir>],
/* FP:hir.rs-0579 */     /// Whether the arguments were written in parenthesized form (e.g., `Fn(T) -> U`).
/* FP:hir.rs-0580 */     ///
/* FP:hir.rs-0581 */     /// This is required mostly for pretty-printing and diagnostics,
/* FP:hir.rs-0582 */     /// but also for changing lifetime elision rules to be "function-like".
/* FP:hir.rs-0583 */     pub parenthesized: GenericArgsParentheses,
/* FP:hir.rs-0584 */     /// The span encompassing the arguments, constraints and the surrounding brackets (`<>` or `()`).
/* FP:hir.rs-0585 */     ///
/* FP:hir.rs-0586 */     /// For example:
/* FP:hir.rs-0587 */     ///
/* FP:hir.rs-0588 */     /// ```ignore (illustrative)
/* FP:hir.rs-0589 */     ///       Foo<A, B, AssocTy = D>           Fn(T, U, V) -> W
/* FP:hir.rs-0590 */     ///          ^^^^^^^^^^^^^^^^^^^             ^^^^^^^^^
/* FP:hir.rs-0591 */     /// ```
/* FP:hir.rs-0592 */     ///
/* FP:hir.rs-0593 */     /// Note that this may be:
/* FP:hir.rs-0594 */     /// - empty, if there are no generic brackets (but there may be hidden lifetimes)
/* FP:hir.rs-0595 */     /// - dummy, if this was generated during desugaring
/* FP:hir.rs-0596 */     pub span_ext: Span,
/* FP:hir.rs-0597 */ }
/* FP:hir.rs-0598 */ 
/* FP:hir.rs-0599 */ impl<'hir> GenericArgs<'hir> {
/* FP:hir.rs-0600 */     pub const fn none() -> Self {
/* FP:hir.rs-0601 */         Self {
/* FP:hir.rs-0602 */             args: &[],
/* FP:hir.rs-0603 */             constraints: &[],
/* FP:hir.rs-0604 */             parenthesized: GenericArgsParentheses::No,
/* FP:hir.rs-0605 */             span_ext: DUMMY_SP,
/* FP:hir.rs-0606 */         }
/* FP:hir.rs-0607 */     }
/* FP:hir.rs-0608 */ 
/* FP:hir.rs-0609 */     /// Obtain the list of input types and the output type if the generic arguments are parenthesized.
/* FP:hir.rs-0610 */     ///
/* FP:hir.rs-0611 */     /// Returns the `Ty0, Ty1, ...` and the `RetTy` in `Trait(Ty0, Ty1, ...) -> RetTy`.
/* FP:hir.rs-0612 */     /// Panics if the parenthesized arguments have an incorrect form (this shouldn't happen).
/* FP:hir.rs-0613 */     pub fn paren_sugar_inputs_output(&self) -> Option<(&[Ty<'hir>], &Ty<'hir>)> {
/* FP:hir.rs-0614 */         if self.parenthesized != GenericArgsParentheses::ParenSugar {
/* FP:hir.rs-0615 */             return None;
/* FP:hir.rs-0616 */         }
/* FP:hir.rs-0617 */ 
/* FP:hir.rs-0618 */         let inputs = self
/* FP:hir.rs-0619 */             .args
/* FP:hir.rs-0620 */             .iter()
/* FP:hir.rs-0621 */             .find_map(|arg| {
/* FP:hir.rs-0622 */                 let GenericArg::Type(ty) = arg else { return None };
/* FP:hir.rs-0623 */                 let TyKind::Tup(tys) = &ty.kind else { return None };
/* FP:hir.rs-0624 */                 Some(tys)
/* FP:hir.rs-0625 */             })
/* FP:hir.rs-0626 */             .unwrap();
/* FP:hir.rs-0627 */ 
/* FP:hir.rs-0628 */         Some((inputs, self.paren_sugar_output_inner()))
/* FP:hir.rs-0629 */     }
/* FP:hir.rs-0630 */ 
/* FP:hir.rs-0631 */     /// Obtain the output type if the generic arguments are parenthesized.
/* FP:hir.rs-0632 */     ///
/* FP:hir.rs-0633 */     /// Returns the `RetTy` in `Trait(Ty0, Ty1, ...) -> RetTy`.
/* FP:hir.rs-0634 */     /// Panics if the parenthesized arguments have an incorrect form (this shouldn't happen).
/* FP:hir.rs-0635 */     pub fn paren_sugar_output(&self) -> Option<&Ty<'hir>> {
/* FP:hir.rs-0636 */         (self.parenthesized == GenericArgsParentheses::ParenSugar)
/* FP:hir.rs-0637 */             .then(|| self.paren_sugar_output_inner())
/* FP:hir.rs-0638 */     }
/* FP:hir.rs-0639 */ 
/* FP:hir.rs-0640 */     fn paren_sugar_output_inner(&self) -> &Ty<'hir> {
/* FP:hir.rs-0641 */         let [constraint] = self.constraints.try_into().unwrap();
/* FP:hir.rs-0642 */         debug_assert_eq!(constraint.ident.name, sym::Output);
/* FP:hir.rs-0643 */         constraint.ty().unwrap()
/* FP:hir.rs-0644 */     }
/* FP:hir.rs-0645 */ 
/* FP:hir.rs-0646 */     pub fn has_err(&self) -> Option<ErrorGuaranteed> {
/* FP:hir.rs-0647 */         self.args
/* FP:hir.rs-0648 */             .iter()
/* FP:hir.rs-0649 */             .find_map(|arg| {
/* FP:hir.rs-0650 */                 let GenericArg::Type(ty) = arg else { return None };
/* FP:hir.rs-0651 */                 let TyKind::Err(guar) = ty.kind else { return None };
/* FP:hir.rs-0652 */                 Some(guar)
/* FP:hir.rs-0653 */             })
/* FP:hir.rs-0654 */             .or_else(|| {
/* FP:hir.rs-0655 */                 self.constraints.iter().find_map(|constraint| {
/* FP:hir.rs-0656 */                     let TyKind::Err(guar) = constraint.ty()?.kind else { return None };
/* FP:hir.rs-0657 */                     Some(guar)
/* FP:hir.rs-0658 */                 })
/* FP:hir.rs-0659 */             })
/* FP:hir.rs-0660 */     }
/* FP:hir.rs-0661 */ 
/* FP:hir.rs-0662 */     #[inline]
/* FP:hir.rs-0663 */     pub fn num_lifetime_params(&self) -> usize {
/* FP:hir.rs-0664 */         self.args.iter().filter(|arg| matches!(arg, GenericArg::Lifetime(_))).count()
/* FP:hir.rs-0665 */     }
/* FP:hir.rs-0666 */ 
/* FP:hir.rs-0667 */     #[inline]
/* FP:hir.rs-0668 */     pub fn has_lifetime_params(&self) -> bool {
/* FP:hir.rs-0669 */         self.args.iter().any(|arg| matches!(arg, GenericArg::Lifetime(_)))
/* FP:hir.rs-0670 */     }
/* FP:hir.rs-0671 */ 
/* FP:hir.rs-0672 */     #[inline]
/* FP:hir.rs-0673 */     /// This function returns the number of type and const generic params.
/* FP:hir.rs-0674 */     /// It should only be used for diagnostics.
/* FP:hir.rs-0675 */     pub fn num_generic_params(&self) -> usize {
/* FP:hir.rs-0676 */         self.args.iter().filter(|arg| !matches!(arg, GenericArg::Lifetime(_))).count()
/* FP:hir.rs-0677 */     }
/* FP:hir.rs-0678 */ 
/* FP:hir.rs-0679 */     /// The span encompassing the arguments and constraints[^1] inside the surrounding brackets.
/* FP:hir.rs-0680 */     ///
/* FP:hir.rs-0681 */     /// Returns `None` if the span is empty (i.e., no brackets) or dummy.
/* FP:hir.rs-0682 */     ///
/* FP:hir.rs-0683 */     /// [^1]: Unless of the form `-> Ty` (see [`GenericArgsParentheses`]).
/* FP:hir.rs-0684 */     pub fn span(&self) -> Option<Span> {
/* FP:hir.rs-0685 */         let span_ext = self.span_ext()?;
/* FP:hir.rs-0686 */         Some(span_ext.with_lo(span_ext.lo() + BytePos(1)).with_hi(span_ext.hi() - BytePos(1)))
/* FP:hir.rs-0687 */     }
/* FP:hir.rs-0688 */ 
/* FP:hir.rs-0689 */     /// Returns span encompassing arguments and their surrounding `<>` or `()`
/* FP:hir.rs-0690 */     pub fn span_ext(&self) -> Option<Span> {
/* FP:hir.rs-0691 */         Some(self.span_ext).filter(|span| !span.is_empty())
/* FP:hir.rs-0692 */     }
/* FP:hir.rs-0693 */ 
/* FP:hir.rs-0694 */     pub fn is_empty(&self) -> bool {
/* FP:hir.rs-0695 */         self.args.is_empty()
/* FP:hir.rs-0696 */     }
/* FP:hir.rs-0697 */ }
/* FP:hir.rs-0698 */ 
/* FP:hir.rs-0699 */ #[derive(Copy, Clone, PartialEq, Eq, Debug, HashStable_Generic)]
/* FP:hir.rs-0700 */ pub enum GenericArgsParentheses {
/* FP:hir.rs-0701 */     No,
/* FP:hir.rs-0702 */     /// Bounds for `feature(return_type_notation)`, like `T: Trait<method(..): Send>`,
/* FP:hir.rs-0703 */     /// where the args are explicitly elided with `..`
/* FP:hir.rs-0704 */     ReturnTypeNotation,
/* FP:hir.rs-0705 */     /// parenthesized function-family traits, like `T: Fn(u32) -> i32`
/* FP:hir.rs-0706 */     ParenSugar,
/* FP:hir.rs-0707 */ }
/* FP:hir.rs-0708 */ 
/* FP:hir.rs-0709 */ /// The modifiers on a trait bound.
/* FP:hir.rs-0710 */ #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, HashStable_Generic)]
/* FP:hir.rs-0711 */ pub struct TraitBoundModifiers {
/* FP:hir.rs-0712 */     pub constness: BoundConstness,
/* FP:hir.rs-0713 */     pub polarity: BoundPolarity,
/* FP:hir.rs-0714 */ }
/* FP:hir.rs-0715 */ 
/* FP:hir.rs-0716 */ impl TraitBoundModifiers {
/* FP:hir.rs-0717 */     pub const NONE: Self =
/* FP:hir.rs-0718 */         TraitBoundModifiers { constness: BoundConstness::Never, polarity: BoundPolarity::Positive };
/* FP:hir.rs-0719 */ }
/* FP:hir.rs-0720 */ 
/* FP:hir.rs-0721 */ #[derive(Clone, Copy, Debug, HashStable_Generic)]
/* FP:hir.rs-0722 */ pub enum GenericBound<'hir> {
/* FP:hir.rs-0723 */     Trait(PolyTraitRef<'hir>),
/* FP:hir.rs-0724 */     Outlives(&'hir Lifetime),
/* FP:hir.rs-0725 */     Use(&'hir [PreciseCapturingArg<'hir>], Span),
/* FP:hir.rs-0726 */ }
/* FP:hir.rs-0727 */ 
/* FP:hir.rs-0728 */ impl GenericBound<'_> {
/* FP:hir.rs-0729 */     pub fn trait_ref(&self) -> Option<&TraitRef<'_>> {
/* FP:hir.rs-0730 */         match self {
/* FP:hir.rs-0731 */             GenericBound::Trait(data) => Some(&data.trait_ref),
/* FP:hir.rs-0732 */             _ => None,
/* FP:hir.rs-0733 */         }
/* FP:hir.rs-0734 */     }
/* FP:hir.rs-0735 */ 
/* FP:hir.rs-0736 */     pub fn span(&self) -> Span {
/* FP:hir.rs-0737 */         match self {
/* FP:hir.rs-0738 */             GenericBound::Trait(t, ..) => t.span,
/* FP:hir.rs-0739 */             GenericBound::Outlives(l) => l.ident.span,
/* FP:hir.rs-0740 */             GenericBound::Use(_, span) => *span,
/* FP:hir.rs-0741 */         }
/* FP:hir.rs-0742 */     }
/* FP:hir.rs-0743 */ }
/* FP:hir.rs-0744 */ 
/* FP:hir.rs-0745 */ pub type GenericBounds<'hir> = &'hir [GenericBound<'hir>];
/* FP:hir.rs-0746 */ 
/* FP:hir.rs-0747 */ #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, HashStable_Generic, Debug)]
/* FP:hir.rs-0748 */ pub enum MissingLifetimeKind {
/* FP:hir.rs-0749 */     /// An explicit `'_`.
/* FP:hir.rs-0750 */     Underscore,
/* FP:hir.rs-0751 */     /// An elided lifetime `&' ty`.
/* FP:hir.rs-0752 */     Ampersand,
/* FP:hir.rs-0753 */     /// An elided lifetime in brackets with written brackets.
/* FP:hir.rs-0754 */     Comma,
/* FP:hir.rs-0755 */     /// An elided lifetime with elided brackets.
/* FP:hir.rs-0756 */     Brackets,
/* FP:hir.rs-0757 */ }
/* FP:hir.rs-0758 */ 
/* FP:hir.rs-0759 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-0760 */ pub enum LifetimeParamKind {
/* FP:hir.rs-0761 */     // Indicates that the lifetime definition was explicitly declared (e.g., in
/* FP:hir.rs-0762 */     // `fn foo<'a>(x: &'a u8) -> &'a u8 { x }`).
/* FP:hir.rs-0763 */     Explicit,
/* FP:hir.rs-0764 */ 
/* FP:hir.rs-0765 */     // Indication that the lifetime was elided (e.g., in both cases in
/* FP:hir.rs-0766 */     // `fn foo(x: &u8) -> &'_ u8 { x }`).
/* FP:hir.rs-0767 */     Elided(MissingLifetimeKind),
/* FP:hir.rs-0768 */ 
/* FP:hir.rs-0769 */     // Indication that the lifetime name was somehow in error.
/* FP:hir.rs-0770 */     Error,
/* FP:hir.rs-0771 */ }
/* FP:hir.rs-0772 */ 
/* FP:hir.rs-0773 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-0774 */ pub enum GenericParamKind<'hir> {
/* FP:hir.rs-0775 */     /// A lifetime definition (e.g., `'a: 'b + 'c + 'd`).
/* FP:hir.rs-0776 */     Lifetime {
/* FP:hir.rs-0777 */         kind: LifetimeParamKind,
/* FP:hir.rs-0778 */     },
/* FP:hir.rs-0779 */     Type {
/* FP:hir.rs-0780 */         default: Option<&'hir Ty<'hir>>,
/* FP:hir.rs-0781 */         synthetic: bool,
/* FP:hir.rs-0782 */     },
/* FP:hir.rs-0783 */     Const {
/* FP:hir.rs-0784 */         ty: &'hir Ty<'hir>,
/* FP:hir.rs-0785 */         /// Optional default value for the const generic param
/* FP:hir.rs-0786 */         default: Option<&'hir ConstArg<'hir>>,
/* FP:hir.rs-0787 */     },
/* FP:hir.rs-0788 */ }
/* FP:hir.rs-0789 */ 
/* FP:hir.rs-0790 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-0791 */ pub struct GenericParam<'hir> {
/* FP:hir.rs-0792 */     #[stable_hasher(ignore)]
/* FP:hir.rs-0793 */     pub hir_id: HirId,
/* FP:hir.rs-0794 */     pub def_id: LocalDefId,
/* FP:hir.rs-0795 */     pub name: ParamName,
/* FP:hir.rs-0796 */     pub span: Span,
/* FP:hir.rs-0797 */     pub pure_wrt_drop: bool,
/* FP:hir.rs-0798 */     pub kind: GenericParamKind<'hir>,
/* FP:hir.rs-0799 */     pub colon_span: Option<Span>,
/* FP:hir.rs-0800 */     pub source: GenericParamSource,
/* FP:hir.rs-0801 */ }
/* FP:hir.rs-0802 */ 
/* FP:hir.rs-0803 */ impl<'hir> GenericParam<'hir> {
/* FP:hir.rs-0804 */     /// Synthetic type-parameters are inserted after normal ones.
/* FP:hir.rs-0805 */     /// In order for normal parameters to be able to refer to synthetic ones,
/* FP:hir.rs-0806 */     /// scans them first.
/* FP:hir.rs-0807 */     pub fn is_impl_trait(&self) -> bool {
/* FP:hir.rs-0808 */         matches!(self.kind, GenericParamKind::Type { synthetic: true, .. })
/* FP:hir.rs-0809 */     }
/* FP:hir.rs-0810 */ 
/* FP:hir.rs-0811 */     /// This can happen for `async fn`, e.g. `async fn f<'_>(&'_ self)`.
/* FP:hir.rs-0812 */     ///
/* FP:hir.rs-0813 */     /// See `lifetime_to_generic_param` in `rustc_ast_lowering` for more information.
/* FP:hir.rs-0814 */     pub fn is_elided_lifetime(&self) -> bool {
/* FP:hir.rs-0815 */         matches!(self.kind, GenericParamKind::Lifetime { kind: LifetimeParamKind::Elided(_) })
/* FP:hir.rs-0816 */     }
/* FP:hir.rs-0817 */ }
/* FP:hir.rs-0818 */ 
/* FP:hir.rs-0819 */ /// Records where the generic parameter originated from.
/* FP:hir.rs-0820 */ ///
/* FP:hir.rs-0821 */ /// This can either be from an item's generics, in which case it's typically
/* FP:hir.rs-0822 */ /// early-bound (but can be a late-bound lifetime in functions, for example),
/* FP:hir.rs-0823 */ /// or from a `for<...>` binder, in which case it's late-bound (and notably,
/* FP:hir.rs-0824 */ /// does not show up in the parent item's generics).
/* FP:hir.rs-0825 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-0826 */ pub enum GenericParamSource {
/* FP:hir.rs-0827 */     // Early or late-bound parameters defined on an item
/* FP:hir.rs-0828 */     Generics,
/* FP:hir.rs-0829 */     // Late-bound parameters defined via a `for<...>`
/* FP:hir.rs-0830 */     Binder,
/* FP:hir.rs-0831 */ }
/* FP:hir.rs-0832 */ 
/* FP:hir.rs-0833 */ #[derive(Default)]
/* FP:hir.rs-0834 */ pub struct GenericParamCount {
/* FP:hir.rs-0835 */     pub lifetimes: usize,
/* FP:hir.rs-0836 */     pub types: usize,
/* FP:hir.rs-0837 */     pub consts: usize,
/* FP:hir.rs-0838 */     pub infer: usize,
/* FP:hir.rs-0839 */ }
/* FP:hir.rs-0840 */ 
/* FP:hir.rs-0841 */ /// Represents lifetimes and type parameters attached to a declaration
/* FP:hir.rs-0842 */ /// of a function, enum, trait, etc.
/* FP:hir.rs-0843 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-0844 */ pub struct Generics<'hir> {
/* FP:hir.rs-0845 */     pub params: &'hir [GenericParam<'hir>],
/* FP:hir.rs-0846 */     pub predicates: &'hir [WherePredicate<'hir>],
/* FP:hir.rs-0847 */     pub has_where_clause_predicates: bool,
/* FP:hir.rs-0848 */     pub where_clause_span: Span,
/* FP:hir.rs-0849 */     pub span: Span,
/* FP:hir.rs-0850 */ }
/* FP:hir.rs-0851 */ 
/* FP:hir.rs-0852 */ impl<'hir> Generics<'hir> {
/* FP:hir.rs-0853 */     pub const fn empty() -> &'hir Generics<'hir> {
/* FP:hir.rs-0854 */         const NOPE: Generics<'_> = Generics {
/* FP:hir.rs-0855 */             params: &[],
/* FP:hir.rs-0856 */             predicates: &[],
/* FP:hir.rs-0857 */             has_where_clause_predicates: false,
/* FP:hir.rs-0858 */             where_clause_span: DUMMY_SP,
/* FP:hir.rs-0859 */             span: DUMMY_SP,
/* FP:hir.rs-0860 */         };
/* FP:hir.rs-0861 */         &NOPE
/* FP:hir.rs-0862 */     }
/* FP:hir.rs-0863 */ 
/* FP:hir.rs-0864 */     pub fn get_named(&self, name: Symbol) -> Option<&GenericParam<'hir>> {
/* FP:hir.rs-0865 */         self.params.iter().find(|&param| name == param.name.ident().name)
/* FP:hir.rs-0866 */     }
/* FP:hir.rs-0867 */ 
/* FP:hir.rs-0868 */     /// If there are generic parameters, return where to introduce a new one.
/* FP:hir.rs-0869 */     pub fn span_for_lifetime_suggestion(&self) -> Option<Span> {
/* FP:hir.rs-0870 */         if let Some(first) = self.params.first()
/* FP:hir.rs-0871 */             && self.span.contains(first.span)
/* FP:hir.rs-0872 */         {
/* FP:hir.rs-0873 */             // `fn foo<A>(t: impl Trait)`
/* FP:hir.rs-0874 */             //         ^ suggest `'a, ` here
/* FP:hir.rs-0875 */             Some(first.span.shrink_to_lo())
/* FP:hir.rs-0876 */         } else {
/* FP:hir.rs-0877 */             None
/* FP:hir.rs-0878 */         }
/* FP:hir.rs-0879 */     }
/* FP:hir.rs-0880 */ 
/* FP:hir.rs-0881 */     /// If there are generic parameters, return where to introduce a new one.
/* FP:hir.rs-0882 */     pub fn span_for_param_suggestion(&self) -> Option<Span> {
/* FP:hir.rs-0883 */         self.params.iter().any(|p| self.span.contains(p.span)).then(|| {
/* FP:hir.rs-0884 */             // `fn foo<A>(t: impl Trait)`
/* FP:hir.rs-0885 */             //          ^ suggest `, T: Trait` here
/* FP:hir.rs-0886 */             self.span.with_lo(self.span.hi() - BytePos(1)).shrink_to_lo()
/* FP:hir.rs-0887 */         })
/* FP:hir.rs-0888 */     }
/* FP:hir.rs-0889 */ 
/* FP:hir.rs-0890 */     /// `Span` where further predicates would be suggested, accounting for trailing commas, like
/* FP:hir.rs-0891 */     ///  in `fn foo<T>(t: T) where T: Foo,` so we don't suggest two trailing commas.
/* FP:hir.rs-0892 */     pub fn tail_span_for_predicate_suggestion(&self) -> Span {
/* FP:hir.rs-0893 */         let end = self.where_clause_span.shrink_to_hi();
/* FP:hir.rs-0894 */         if self.has_where_clause_predicates {
/* FP:hir.rs-0895 */             self.predicates
/* FP:hir.rs-0896 */                 .iter()
/* FP:hir.rs-0897 */                 .rfind(|&p| p.kind.in_where_clause())
/* FP:hir.rs-0898 */                 .map_or(end, |p| p.span)
/* FP:hir.rs-0899 */                 .shrink_to_hi()
/* FP:hir.rs-0900 */                 .to(end)
/* FP:hir.rs-0901 */         } else {
/* FP:hir.rs-0902 */             end
/* FP:hir.rs-0903 */         }
/* FP:hir.rs-0904 */     }
/* FP:hir.rs-0905 */ 
/* FP:hir.rs-0906 */     pub fn add_where_or_trailing_comma(&self) -> &'static str {
/* FP:hir.rs-0907 */         if self.has_where_clause_predicates {
/* FP:hir.rs-0908 */             ","
/* FP:hir.rs-0909 */         } else if self.where_clause_span.is_empty() {
/* FP:hir.rs-0910 */             " where"
/* FP:hir.rs-0911 */         } else {
/* FP:hir.rs-0912 */             // No where clause predicates, but we have `where` token
/* FP:hir.rs-0913 */             ""
/* FP:hir.rs-0914 */         }
/* FP:hir.rs-0915 */     }
/* FP:hir.rs-0916 */ 
/* FP:hir.rs-0917 */     pub fn bounds_for_param(
/* FP:hir.rs-0918 */         &self,
/* FP:hir.rs-0919 */         param_def_id: LocalDefId,
/* FP:hir.rs-0920 */     ) -> impl Iterator<Item = &WhereBoundPredicate<'hir>> {
/* FP:hir.rs-0921 */         self.predicates.iter().filter_map(move |pred| match pred.kind {
/* FP:hir.rs-0922 */             WherePredicateKind::BoundPredicate(bp)
/* FP:hir.rs-0923 */                 if bp.is_param_bound(param_def_id.to_def_id()) =>
/* FP:hir.rs-0924 */             {
/* FP:hir.rs-0925 */                 Some(bp)
/* FP:hir.rs-0926 */             }
/* FP:hir.rs-0927 */             _ => None,
/* FP:hir.rs-0928 */         })
/* FP:hir.rs-0929 */     }
/* FP:hir.rs-0930 */ 
/* FP:hir.rs-0931 */     pub fn outlives_for_param(
/* FP:hir.rs-0932 */         &self,
/* FP:hir.rs-0933 */         param_def_id: LocalDefId,
/* FP:hir.rs-0934 */     ) -> impl Iterator<Item = &WhereRegionPredicate<'_>> {
/* FP:hir.rs-0935 */         self.predicates.iter().filter_map(move |pred| match pred.kind {
/* FP:hir.rs-0936 */             WherePredicateKind::RegionPredicate(rp) if rp.is_param_bound(param_def_id) => Some(rp),
/* FP:hir.rs-0937 */             _ => None,
/* FP:hir.rs-0938 */         })
/* FP:hir.rs-0939 */     }
/* FP:hir.rs-0940 */ 
/* FP:hir.rs-0941 */     /// Returns a suggestable empty span right after the "final" bound of the generic parameter.
/* FP:hir.rs-0942 */     ///
/* FP:hir.rs-0943 */     /// If that bound needs to be wrapped in parentheses to avoid ambiguity with
/* FP:hir.rs-0944 */     /// subsequent bounds, it also returns an empty span for an open parenthesis
/* FP:hir.rs-0945 */     /// as the second component.
/* FP:hir.rs-0946 */     ///
/* FP:hir.rs-0947 */     /// E.g., adding `+ 'static` after `Fn() -> dyn Future<Output = ()>` or
/* FP:hir.rs-0948 */     /// `Fn() -> &'static dyn Debug` requires parentheses:
/* FP:hir.rs-0949 */     /// `Fn() -> (dyn Future<Output = ()>) + 'static` and
/* FP:hir.rs-0950 */     /// `Fn() -> &'static (dyn Debug) + 'static`, respectively.
/* FP:hir.rs-0951 */     pub fn bounds_span_for_suggestions(
/* FP:hir.rs-0952 */         &self,
/* FP:hir.rs-0953 */         param_def_id: LocalDefId,
/* FP:hir.rs-0954 */     ) -> Option<(Span, Option<Span>)> {
/* FP:hir.rs-0955 */         self.bounds_for_param(param_def_id).flat_map(|bp| bp.bounds.iter().rev()).find_map(
/* FP:hir.rs-0956 */             |bound| {
/* FP:hir.rs-0957 */                 let span_for_parentheses = if let Some(trait_ref) = bound.trait_ref()
/* FP:hir.rs-0958 */                     && let [.., segment] = trait_ref.path.segments
/* FP:hir.rs-0959 */                     && let Some(ret_ty) = segment.args().paren_sugar_output()
/* FP:hir.rs-0960 */                     && let ret_ty = ret_ty.peel_refs()
/* FP:hir.rs-0961 */                     && let TyKind::TraitObject(_, tagged_ptr) = ret_ty.kind
/* FP:hir.rs-0962 */                     && let TraitObjectSyntax::Dyn = tagged_ptr.tag()
/* FP:hir.rs-0963 */                     && ret_ty.span.can_be_used_for_suggestions()
/* FP:hir.rs-0964 */                 {
/* FP:hir.rs-0965 */                     Some(ret_ty.span)
/* FP:hir.rs-0966 */                 } else {
/* FP:hir.rs-0967 */                     None
/* FP:hir.rs-0968 */                 };
/* FP:hir.rs-0969 */ 
/* FP:hir.rs-0970 */                 span_for_parentheses.map_or_else(
/* FP:hir.rs-0971 */                     || {
/* FP:hir.rs-0972 */                         // We include bounds that come from a `#[derive(_)]` but point at the user's code,
/* FP:hir.rs-0973 */                         // as we use this method to get a span appropriate for suggestions.
/* FP:hir.rs-0974 */                         let bs = bound.span();
/* FP:hir.rs-0975 */                         bs.can_be_used_for_suggestions().then(|| (bs.shrink_to_hi(), None))
/* FP:hir.rs-0976 */                     },
/* FP:hir.rs-0977 */                     |span| Some((span.shrink_to_hi(), Some(span.shrink_to_lo()))),
/* FP:hir.rs-0978 */                 )
/* FP:hir.rs-0979 */             },
/* FP:hir.rs-0980 */         )
/* FP:hir.rs-0981 */     }
/* FP:hir.rs-0982 */ 
/* FP:hir.rs-0983 */     pub fn span_for_predicate_removal(&self, pos: usize) -> Span {
/* FP:hir.rs-0984 */         let predicate = &self.predicates[pos];
/* FP:hir.rs-0985 */         let span = predicate.span;
/* FP:hir.rs-0986 */ 
/* FP:hir.rs-0987 */         if !predicate.kind.in_where_clause() {
/* FP:hir.rs-0988 */             // <T: ?Sized, U>
/* FP:hir.rs-0989 */             //   ^^^^^^^^
/* FP:hir.rs-0990 */             return span;
/* FP:hir.rs-0991 */         }
/* FP:hir.rs-0992 */ 
/* FP:hir.rs-0993 */         // We need to find out which comma to remove.
/* FP:hir.rs-0994 */         if pos < self.predicates.len() - 1 {
/* FP:hir.rs-0995 */             let next_pred = &self.predicates[pos + 1];
/* FP:hir.rs-0996 */             if next_pred.kind.in_where_clause() {
/* FP:hir.rs-0997 */                 // where T: ?Sized, Foo: Bar,
/* FP:hir.rs-0998 */                 //       ^^^^^^^^^^^
/* FP:hir.rs-0999 */                 return span.until(next_pred.span);
/* FP:hir.rs-1000 */             }
/* FP:hir.rs-1001 */         }
/* FP:hir.rs-1002 */ 
/* FP:hir.rs-1003 */         if pos > 0 {
/* FP:hir.rs-1004 */             let prev_pred = &self.predicates[pos - 1];
/* FP:hir.rs-1005 */             if prev_pred.kind.in_where_clause() {
/* FP:hir.rs-1006 */                 // where Foo: Bar, T: ?Sized,
/* FP:hir.rs-1007 */                 //               ^^^^^^^^^^^
/* FP:hir.rs-1008 */                 return prev_pred.span.shrink_to_hi().to(span);
/* FP:hir.rs-1009 */             }
/* FP:hir.rs-1010 */         }
/* FP:hir.rs-1011 */ 
/* FP:hir.rs-1012 */         // This is the only predicate in the where clause.
/* FP:hir.rs-1013 */         // where T: ?Sized
/* FP:hir.rs-1014 */         // ^^^^^^^^^^^^^^^
/* FP:hir.rs-1015 */         self.where_clause_span
/* FP:hir.rs-1016 */     }
/* FP:hir.rs-1017 */ 
/* FP:hir.rs-1018 */     pub fn span_for_bound_removal(&self, predicate_pos: usize, bound_pos: usize) -> Span {
/* FP:hir.rs-1019 */         let predicate = &self.predicates[predicate_pos];
/* FP:hir.rs-1020 */         let bounds = predicate.kind.bounds();
/* FP:hir.rs-1021 */ 
/* FP:hir.rs-1022 */         if bounds.len() == 1 {
/* FP:hir.rs-1023 */             return self.span_for_predicate_removal(predicate_pos);
/* FP:hir.rs-1024 */         }
/* FP:hir.rs-1025 */ 
/* FP:hir.rs-1026 */         let bound_span = bounds[bound_pos].span();
/* FP:hir.rs-1027 */         if bound_pos < bounds.len() - 1 {
/* FP:hir.rs-1028 */             // If there's another bound after the current bound
/* FP:hir.rs-1029 */             // include the following '+' e.g.:
/* FP:hir.rs-1030 */             //
/* FP:hir.rs-1031 */             //  `T: Foo + CurrentBound + Bar`
/* FP:hir.rs-1032 */             //            ^^^^^^^^^^^^^^^
/* FP:hir.rs-1033 */             bound_span.to(bounds[bound_pos + 1].span().shrink_to_lo())
/* FP:hir.rs-1034 */         } else {
/* FP:hir.rs-1035 */             // If the current bound is the last bound
/* FP:hir.rs-1036 */             // include the preceding '+' E.g.:
/* FP:hir.rs-1037 */             //
/* FP:hir.rs-1038 */             //  `T: Foo + Bar + CurrentBound`
/* FP:hir.rs-1039 */             //               ^^^^^^^^^^^^^^^
/* FP:hir.rs-1040 */             bound_span.with_lo(bounds[bound_pos - 1].span().hi())
/* FP:hir.rs-1041 */         }
/* FP:hir.rs-1042 */     }
/* FP:hir.rs-1043 */ }
/* FP:hir.rs-1044 */ 
/* FP:hir.rs-1045 */ /// A single predicate in a where-clause.
/* FP:hir.rs-1046 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1047 */ pub struct WherePredicate<'hir> {
/* FP:hir.rs-1048 */     #[stable_hasher(ignore)]
/* FP:hir.rs-1049 */     pub hir_id: HirId,
/* FP:hir.rs-1050 */     pub span: Span,
/* FP:hir.rs-1051 */     pub kind: &'hir WherePredicateKind<'hir>,
/* FP:hir.rs-1052 */ }
/* FP:hir.rs-1053 */ 
/* FP:hir.rs-1054 */ /// The kind of a single predicate in a where-clause.
/* FP:hir.rs-1055 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1056 */ pub enum WherePredicateKind<'hir> {
/* FP:hir.rs-1057 */     /// A type bound (e.g., `for<'c> Foo: Send + Clone + 'c`).
/* FP:hir.rs-1058 */     BoundPredicate(WhereBoundPredicate<'hir>),
/* FP:hir.rs-1059 */     /// A lifetime predicate (e.g., `'a: 'b + 'c`).
/* FP:hir.rs-1060 */     RegionPredicate(WhereRegionPredicate<'hir>),
/* FP:hir.rs-1061 */     /// An equality predicate (unsupported).
/* FP:hir.rs-1062 */     EqPredicate(WhereEqPredicate<'hir>),
/* FP:hir.rs-1063 */ }
/* FP:hir.rs-1064 */ 
/* FP:hir.rs-1065 */ impl<'hir> WherePredicateKind<'hir> {
/* FP:hir.rs-1066 */     pub fn in_where_clause(&self) -> bool {
/* FP:hir.rs-1067 */         match self {
/* FP:hir.rs-1068 */             WherePredicateKind::BoundPredicate(p) => p.origin == PredicateOrigin::WhereClause,
/* FP:hir.rs-1069 */             WherePredicateKind::RegionPredicate(p) => p.in_where_clause,
/* FP:hir.rs-1070 */             WherePredicateKind::EqPredicate(_) => false,
/* FP:hir.rs-1071 */         }
/* FP:hir.rs-1072 */     }
/* FP:hir.rs-1073 */ 
/* FP:hir.rs-1074 */     pub fn bounds(&self) -> GenericBounds<'hir> {
/* FP:hir.rs-1075 */         match self {
/* FP:hir.rs-1076 */             WherePredicateKind::BoundPredicate(p) => p.bounds,
/* FP:hir.rs-1077 */             WherePredicateKind::RegionPredicate(p) => p.bounds,
/* FP:hir.rs-1078 */             WherePredicateKind::EqPredicate(_) => &[],
/* FP:hir.rs-1079 */         }
/* FP:hir.rs-1080 */     }
/* FP:hir.rs-1081 */ }
/* FP:hir.rs-1082 */ 
/* FP:hir.rs-1083 */ #[derive(Copy, Clone, Debug, HashStable_Generic, PartialEq, Eq)]
/* FP:hir.rs-1084 */ pub enum PredicateOrigin {
/* FP:hir.rs-1085 */     WhereClause,
/* FP:hir.rs-1086 */     GenericParam,
/* FP:hir.rs-1087 */     ImplTrait,
/* FP:hir.rs-1088 */ }
/* FP:hir.rs-1089 */ 
/* FP:hir.rs-1090 */ /// A type bound (e.g., `for<'c> Foo: Send + Clone + 'c`).
/* FP:hir.rs-1091 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1092 */ pub struct WhereBoundPredicate<'hir> {
/* FP:hir.rs-1093 */     /// Origin of the predicate.
/* FP:hir.rs-1094 */     pub origin: PredicateOrigin,
/* FP:hir.rs-1095 */     /// Any generics from a `for` binding.
/* FP:hir.rs-1096 */     pub bound_generic_params: &'hir [GenericParam<'hir>],
/* FP:hir.rs-1097 */     /// The type being bounded.
/* FP:hir.rs-1098 */     pub bounded_ty: &'hir Ty<'hir>,
/* FP:hir.rs-1099 */     /// Trait and lifetime bounds (e.g., `Clone + Send + 'static`).
/* FP:hir.rs-1100 */     pub bounds: GenericBounds<'hir>,
/* FP:hir.rs-1101 */ }
/* FP:hir.rs-1102 */ 
/* FP:hir.rs-1103 */ impl<'hir> WhereBoundPredicate<'hir> {
/* FP:hir.rs-1104 */     /// Returns `true` if `param_def_id` matches the `bounded_ty` of this predicate.
/* FP:hir.rs-1105 */     pub fn is_param_bound(&self, param_def_id: DefId) -> bool {
/* FP:hir.rs-1106 */         self.bounded_ty.as_generic_param().is_some_and(|(def_id, _)| def_id == param_def_id)
/* FP:hir.rs-1107 */     }
/* FP:hir.rs-1108 */ }
/* FP:hir.rs-1109 */ 
/* FP:hir.rs-1110 */ /// A lifetime predicate (e.g., `'a: 'b + 'c`).
/* FP:hir.rs-1111 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1112 */ pub struct WhereRegionPredicate<'hir> {
/* FP:hir.rs-1113 */     pub in_where_clause: bool,
/* FP:hir.rs-1114 */     pub lifetime: &'hir Lifetime,
/* FP:hir.rs-1115 */     pub bounds: GenericBounds<'hir>,
/* FP:hir.rs-1116 */ }
/* FP:hir.rs-1117 */ 
/* FP:hir.rs-1118 */ impl<'hir> WhereRegionPredicate<'hir> {
/* FP:hir.rs-1119 */     /// Returns `true` if `param_def_id` matches the `lifetime` of this predicate.
/* FP:hir.rs-1120 */     fn is_param_bound(&self, param_def_id: LocalDefId) -> bool {
/* FP:hir.rs-1121 */         self.lifetime.kind == LifetimeKind::Param(param_def_id)
/* FP:hir.rs-1122 */     }
/* FP:hir.rs-1123 */ }
/* FP:hir.rs-1124 */ 
/* FP:hir.rs-1125 */ /// An equality predicate (e.g., `T = int`); currently unsupported.
/* FP:hir.rs-1126 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1127 */ pub struct WhereEqPredicate<'hir> {
/* FP:hir.rs-1128 */     pub lhs_ty: &'hir Ty<'hir>,
/* FP:hir.rs-1129 */     pub rhs_ty: &'hir Ty<'hir>,
/* FP:hir.rs-1130 */ }
/* FP:hir.rs-1131 */ 
/* FP:hir.rs-1132 */ /// HIR node coupled with its parent's id in the same HIR owner.
/* FP:hir.rs-1133 */ ///
/* FP:hir.rs-1134 */ /// The parent is trash when the node is a HIR owner.
/* FP:hir.rs-1135 */ #[derive(Clone, Copy, Debug)]
/* FP:hir.rs-1136 */ pub struct ParentedNode<'tcx> {
/* FP:hir.rs-1137 */     pub parent: ItemLocalId,
/* FP:hir.rs-1138 */     pub node: Node<'tcx>,
/* FP:hir.rs-1139 */ }
/* FP:hir.rs-1140 */ 
/* FP:hir.rs-1141 */ /// Arguments passed to an attribute macro.
/* FP:hir.rs-1142 */ #[derive(Clone, Debug, HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-1143 */ pub enum AttrArgs {
/* FP:hir.rs-1144 */     /// No arguments: `#[attr]`.
/* FP:hir.rs-1145 */     Empty,
/* FP:hir.rs-1146 */     /// Delimited arguments: `#[attr()/[]/{}]`.
/* FP:hir.rs-1147 */     Delimited(DelimArgs),
/* FP:hir.rs-1148 */     /// Arguments of a key-value attribute: `#[attr = "value"]`.
/* FP:hir.rs-1149 */     Eq {
/* FP:hir.rs-1150 */         /// Span of the `=` token.
/* FP:hir.rs-1151 */         eq_span: Span,
/* FP:hir.rs-1152 */         /// The "value".
/* FP:hir.rs-1153 */         expr: MetaItemLit,
/* FP:hir.rs-1154 */     },
/* FP:hir.rs-1155 */ }
/* FP:hir.rs-1156 */ 
/* FP:hir.rs-1157 */ #[derive(Clone, Debug, HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-1158 */ pub struct AttrPath {
/* FP:hir.rs-1159 */     pub segments: Box<[Ident]>,
/* FP:hir.rs-1160 */     pub span: Span,
/* FP:hir.rs-1161 */ }
/* FP:hir.rs-1162 */ 
/* FP:hir.rs-1163 */ impl IntoDiagArg for AttrPath {
/* FP:hir.rs-1164 */     fn into_diag_arg(self, path: &mut Option<std::path::PathBuf>) -> DiagArgValue {
/* FP:hir.rs-1165 */         self.to_string().into_diag_arg(path)
/* FP:hir.rs-1166 */     }
/* FP:hir.rs-1167 */ }
/* FP:hir.rs-1168 */ 
/* FP:hir.rs-1169 */ impl AttrPath {
/* FP:hir.rs-1170 */     pub fn from_ast(path: &ast::Path) -> Self {
/* FP:hir.rs-1171 */         AttrPath {
/* FP:hir.rs-1172 */             segments: path.segments.iter().map(|i| i.ident).collect::<Vec<_>>().into_boxed_slice(),
/* FP:hir.rs-1173 */             span: path.span,
/* FP:hir.rs-1174 */         }
/* FP:hir.rs-1175 */     }
/* FP:hir.rs-1176 */ }
/* FP:hir.rs-1177 */ 
/* FP:hir.rs-1178 */ impl fmt::Display for AttrPath {
/* FP:hir.rs-1179 */     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/* FP:hir.rs-1180 */         write!(f, "{}", join_path_idents(&self.segments))
/* FP:hir.rs-1181 */     }
/* FP:hir.rs-1182 */ }
/* FP:hir.rs-1183 */ 
/* FP:hir.rs-1184 */ #[derive(Clone, Debug, HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-1185 */ pub struct AttrItem {
/* FP:hir.rs-1186 */     // Not lowered to hir::Path because we have no NodeId to resolve to.
/* FP:hir.rs-1187 */     pub path: AttrPath,
/* FP:hir.rs-1188 */     pub args: AttrArgs,
/* FP:hir.rs-1189 */     pub id: HashIgnoredAttrId,
/* FP:hir.rs-1190 */     /// Denotes if the attribute decorates the following construct (outer)
/* FP:hir.rs-1191 */     /// or the construct this attribute is contained within (inner).
/* FP:hir.rs-1192 */     pub style: AttrStyle,
/* FP:hir.rs-1193 */     /// Span of the entire attribute
/* FP:hir.rs-1194 */     pub span: Span,
/* FP:hir.rs-1195 */ }
/* FP:hir.rs-1196 */ 
/* FP:hir.rs-1197 */ /// The derived implementation of [`HashStable_Generic`] on [`Attribute`]s shouldn't hash
/* FP:hir.rs-1198 */ /// [`AttrId`]s. By wrapping them in this, we make sure we never do.
/* FP:hir.rs-1199 */ #[derive(Copy, Debug, Encodable, Decodable, Clone)]
/* FP:hir.rs-1200 */ pub struct HashIgnoredAttrId {
/* FP:hir.rs-1201 */     pub attr_id: AttrId,
/* FP:hir.rs-1202 */ }
/* FP:hir.rs-1203 */ 
/* FP:hir.rs-1204 */ #[derive(Clone, Debug, Encodable, Decodable, HashStable_Generic)]
/* FP:hir.rs-1205 */ pub enum Attribute {
/* FP:hir.rs-1206 */     /// A parsed built-in attribute.
/* FP:hir.rs-1207 */     ///
/* FP:hir.rs-1208 */     /// Each attribute has a span connected to it. However, you must be somewhat careful using it.
/* FP:hir.rs-1209 */     /// That's because sometimes we merge multiple attributes together, like when an item has
/* FP:hir.rs-1210 */     /// multiple `repr` attributes. In this case the span might not be very useful.
/* FP:hir.rs-1211 */     Parsed(AttributeKind),
/* FP:hir.rs-1212 */ 
/* FP:hir.rs-1213 */     /// An attribute that could not be parsed, out of a token-like representation.
/* FP:hir.rs-1214 */     /// This is the case for custom tool attributes.
/* FP:hir.rs-1215 */     Unparsed(Box<AttrItem>),
/* FP:hir.rs-1216 */ }
/* FP:hir.rs-1217 */ 
/* FP:hir.rs-1218 */ impl Attribute {
/* FP:hir.rs-1219 */     pub fn get_normal_item(&self) -> &AttrItem {
/* FP:hir.rs-1220 */         match &self {
/* FP:hir.rs-1221 */             Attribute::Unparsed(normal) => &normal,
/* FP:hir.rs-1222 */             _ => panic!("unexpected parsed attribute"),
/* FP:hir.rs-1223 */         }
/* FP:hir.rs-1224 */     }
/* FP:hir.rs-1225 */ 
/* FP:hir.rs-1226 */     pub fn unwrap_normal_item(self) -> AttrItem {
/* FP:hir.rs-1227 */         match self {
/* FP:hir.rs-1228 */             Attribute::Unparsed(normal) => *normal,
/* FP:hir.rs-1229 */             _ => panic!("unexpected parsed attribute"),
/* FP:hir.rs-1230 */         }
/* FP:hir.rs-1231 */     }
/* FP:hir.rs-1232 */ 
/* FP:hir.rs-1233 */     pub fn value_lit(&self) -> Option<&MetaItemLit> {
/* FP:hir.rs-1234 */         match &self {
/* FP:hir.rs-1235 */             Attribute::Unparsed(n) => match n.as_ref() {
/* FP:hir.rs-1236 */                 AttrItem { args: AttrArgs::Eq { eq_span: _, expr }, .. } => Some(expr),
/* FP:hir.rs-1237 */                 _ => None,
/* FP:hir.rs-1238 */             },
/* FP:hir.rs-1239 */             _ => None,
/* FP:hir.rs-1240 */         }
/* FP:hir.rs-1241 */     }
/* FP:hir.rs-1242 */ 
/* FP:hir.rs-1243 */     pub fn is_parsed_attr(&self) -> bool {
/* FP:hir.rs-1244 */         match self {
/* FP:hir.rs-1245 */             Attribute::Parsed(_) => true,
/* FP:hir.rs-1246 */             Attribute::Unparsed(_) => false,
/* FP:hir.rs-1247 */         }
/* FP:hir.rs-1248 */     }
/* FP:hir.rs-1249 */ }
/* FP:hir.rs-1250 */ 
/* FP:hir.rs-1251 */ impl AttributeExt for Attribute {
/* FP:hir.rs-1252 */     #[inline]
/* FP:hir.rs-1253 */     fn id(&self) -> AttrId {
/* FP:hir.rs-1254 */         match &self {
/* FP:hir.rs-1255 */             Attribute::Unparsed(u) => u.id.attr_id,
/* FP:hir.rs-1256 */             _ => panic!(),
/* FP:hir.rs-1257 */         }
/* FP:hir.rs-1258 */     }
/* FP:hir.rs-1259 */ 
/* FP:hir.rs-1260 */     #[inline]
/* FP:hir.rs-1261 */     fn meta_item_list(&self) -> Option<ThinVec<ast::MetaItemInner>> {
/* FP:hir.rs-1262 */         match &self {
/* FP:hir.rs-1263 */             Attribute::Unparsed(n) => match n.as_ref() {
/* FP:hir.rs-1264 */                 AttrItem { args: AttrArgs::Delimited(d), .. } => {
/* FP:hir.rs-1265 */                     ast::MetaItemKind::list_from_tokens(d.tokens.clone())
/* FP:hir.rs-1266 */                 }
/* FP:hir.rs-1267 */                 _ => None,
/* FP:hir.rs-1268 */             },
/* FP:hir.rs-1269 */             _ => None,
/* FP:hir.rs-1270 */         }
/* FP:hir.rs-1271 */     }
/* FP:hir.rs-1272 */ 
/* FP:hir.rs-1273 */     #[inline]
/* FP:hir.rs-1274 */     fn value_str(&self) -> Option<Symbol> {
/* FP:hir.rs-1275 */         self.value_lit().and_then(|x| x.value_str())
/* FP:hir.rs-1276 */     }
/* FP:hir.rs-1277 */ 
/* FP:hir.rs-1278 */     #[inline]
/* FP:hir.rs-1279 */     fn value_span(&self) -> Option<Span> {
/* FP:hir.rs-1280 */         self.value_lit().map(|i| i.span)
/* FP:hir.rs-1281 */     }
/* FP:hir.rs-1282 */ 
/* FP:hir.rs-1283 */     /// For a single-segment attribute, returns its name; otherwise, returns `None`.
/* FP:hir.rs-1284 */     #[inline]
/* FP:hir.rs-1285 */     fn ident(&self) -> Option<Ident> {
/* FP:hir.rs-1286 */         match &self {
/* FP:hir.rs-1287 */             Attribute::Unparsed(n) => {
/* FP:hir.rs-1288 */                 if let [ident] = n.path.segments.as_ref() {
/* FP:hir.rs-1289 */                     Some(*ident)
/* FP:hir.rs-1290 */                 } else {
/* FP:hir.rs-1291 */                     None
/* FP:hir.rs-1292 */                 }
/* FP:hir.rs-1293 */             }
/* FP:hir.rs-1294 */             _ => None,
/* FP:hir.rs-1295 */         }
/* FP:hir.rs-1296 */     }
/* FP:hir.rs-1297 */ 
/* FP:hir.rs-1298 */     #[inline]
/* FP:hir.rs-1299 */     fn path_matches(&self, name: &[Symbol]) -> bool {
/* FP:hir.rs-1300 */         match &self {
/* FP:hir.rs-1301 */             Attribute::Unparsed(n) => {
/* FP:hir.rs-1302 */                 n.path.segments.len() == name.len()
/* FP:hir.rs-1303 */                     && n.path.segments.iter().zip(name).all(|(s, n)| s.name == *n)
/* FP:hir.rs-1304 */             }
/* FP:hir.rs-1305 */             _ => false,
/* FP:hir.rs-1306 */         }
/* FP:hir.rs-1307 */     }
/* FP:hir.rs-1308 */ 
/* FP:hir.rs-1309 */     #[inline]
/* FP:hir.rs-1310 */     fn is_doc_comment(&self) -> bool {
/* FP:hir.rs-1311 */         matches!(self, Attribute::Parsed(AttributeKind::DocComment { .. }))
/* FP:hir.rs-1312 */     }
/* FP:hir.rs-1313 */ 
/* FP:hir.rs-1314 */     #[inline]
/* FP:hir.rs-1315 */     fn span(&self) -> Span {
/* FP:hir.rs-1316 */         match &self {
/* FP:hir.rs-1317 */             Attribute::Unparsed(u) => u.span,
/* FP:hir.rs-1318 */             // FIXME: should not be needed anymore when all attrs are parsed
/* FP:hir.rs-1319 */             Attribute::Parsed(AttributeKind::DocComment { span, .. }) => *span,
/* FP:hir.rs-1320 */             Attribute::Parsed(AttributeKind::Deprecation { span, .. }) => *span,
/* FP:hir.rs-1321 */             Attribute::Parsed(AttributeKind::AllowInternalUnsafe(span)) => *span,
/* FP:hir.rs-1322 */             Attribute::Parsed(AttributeKind::Linkage(_, span)) => *span,
/* FP:hir.rs-1323 */             a => panic!("can't get the span of an arbitrary parsed attribute: {a:?}"),
/* FP:hir.rs-1324 */         }
/* FP:hir.rs-1325 */     }
/* FP:hir.rs-1326 */ 
/* FP:hir.rs-1327 */     #[inline]
/* FP:hir.rs-1328 */     fn is_word(&self) -> bool {
/* FP:hir.rs-1329 */         match &self {
/* FP:hir.rs-1330 */             Attribute::Unparsed(n) => {
/* FP:hir.rs-1331 */                 matches!(n.args, AttrArgs::Empty)
/* FP:hir.rs-1332 */             }
/* FP:hir.rs-1333 */             _ => false,
/* FP:hir.rs-1334 */         }
/* FP:hir.rs-1335 */     }
/* FP:hir.rs-1336 */ 
/* FP:hir.rs-1337 */     #[inline]
/* FP:hir.rs-1338 */     fn ident_path(&self) -> Option<SmallVec<[Ident; 1]>> {
/* FP:hir.rs-1339 */         match &self {
/* FP:hir.rs-1340 */             Attribute::Unparsed(n) => Some(n.path.segments.iter().copied().collect()),
/* FP:hir.rs-1341 */             _ => None,
/* FP:hir.rs-1342 */         }
/* FP:hir.rs-1343 */     }
/* FP:hir.rs-1344 */ 
/* FP:hir.rs-1345 */     #[inline]
/* FP:hir.rs-1346 */     fn doc_str(&self) -> Option<Symbol> {
/* FP:hir.rs-1347 */         match &self {
/* FP:hir.rs-1348 */             Attribute::Parsed(AttributeKind::DocComment { comment, .. }) => Some(*comment),
/* FP:hir.rs-1349 */             Attribute::Unparsed(_) if self.has_name(sym::doc) => self.value_str(),
/* FP:hir.rs-1350 */             _ => None,
/* FP:hir.rs-1351 */         }
/* FP:hir.rs-1352 */     }
/* FP:hir.rs-1353 */ 
/* FP:hir.rs-1354 */     fn is_automatically_derived_attr(&self) -> bool {
/* FP:hir.rs-1355 */         matches!(self, Attribute::Parsed(AttributeKind::AutomaticallyDerived(..)))
/* FP:hir.rs-1356 */     }
/* FP:hir.rs-1357 */ 
/* FP:hir.rs-1358 */     #[inline]
/* FP:hir.rs-1359 */     fn doc_str_and_comment_kind(&self) -> Option<(Symbol, CommentKind)> {
/* FP:hir.rs-1360 */         match &self {
/* FP:hir.rs-1361 */             Attribute::Parsed(AttributeKind::DocComment { kind, comment, .. }) => {
/* FP:hir.rs-1362 */                 Some((*comment, *kind))
/* FP:hir.rs-1363 */             }
/* FP:hir.rs-1364 */             Attribute::Unparsed(_) if self.has_name(sym::doc) => {
/* FP:hir.rs-1365 */                 self.value_str().map(|s| (s, CommentKind::Line))
/* FP:hir.rs-1366 */             }
/* FP:hir.rs-1367 */             _ => None,
/* FP:hir.rs-1368 */         }
/* FP:hir.rs-1369 */     }
/* FP:hir.rs-1370 */ 
/* FP:hir.rs-1371 */     fn doc_resolution_scope(&self) -> Option<AttrStyle> {
/* FP:hir.rs-1372 */         match self {
/* FP:hir.rs-1373 */             Attribute::Parsed(AttributeKind::DocComment { style, .. }) => Some(*style),
/* FP:hir.rs-1374 */             Attribute::Unparsed(attr) if self.has_name(sym::doc) && self.value_str().is_some() => {
/* FP:hir.rs-1375 */                 Some(attr.style)
/* FP:hir.rs-1376 */             }
/* FP:hir.rs-1377 */             _ => None,
/* FP:hir.rs-1378 */         }
/* FP:hir.rs-1379 */     }
/* FP:hir.rs-1380 */ 
/* FP:hir.rs-1381 */     fn is_proc_macro_attr(&self) -> bool {
/* FP:hir.rs-1382 */         matches!(
/* FP:hir.rs-1383 */             self,
/* FP:hir.rs-1384 */             Attribute::Parsed(
/* FP:hir.rs-1385 */                 AttributeKind::ProcMacro(..)
/* FP:hir.rs-1386 */                     | AttributeKind::ProcMacroAttribute(..)
/* FP:hir.rs-1387 */                     | AttributeKind::ProcMacroDerive { .. }
/* FP:hir.rs-1388 */             )
/* FP:hir.rs-1389 */         )
/* FP:hir.rs-1390 */     }
/* FP:hir.rs-1391 */ }
/* FP:hir.rs-1392 */ 
/* FP:hir.rs-1393 */ // FIXME(fn_delegation): use function delegation instead of manually forwarding
/* FP:hir.rs-1394 */ impl Attribute {
/* FP:hir.rs-1395 */     #[inline]
/* FP:hir.rs-1396 */     pub fn id(&self) -> AttrId {
/* FP:hir.rs-1397 */         AttributeExt::id(self)
/* FP:hir.rs-1398 */     }
/* FP:hir.rs-1399 */ 
/* FP:hir.rs-1400 */     #[inline]
/* FP:hir.rs-1401 */     pub fn name(&self) -> Option<Symbol> {
/* FP:hir.rs-1402 */         AttributeExt::name(self)
/* FP:hir.rs-1403 */     }
/* FP:hir.rs-1404 */ 
/* FP:hir.rs-1405 */     #[inline]
/* FP:hir.rs-1406 */     pub fn meta_item_list(&self) -> Option<ThinVec<MetaItemInner>> {
/* FP:hir.rs-1407 */         AttributeExt::meta_item_list(self)
/* FP:hir.rs-1408 */     }
/* FP:hir.rs-1409 */ 
/* FP:hir.rs-1410 */     #[inline]
/* FP:hir.rs-1411 */     pub fn value_str(&self) -> Option<Symbol> {
/* FP:hir.rs-1412 */         AttributeExt::value_str(self)
/* FP:hir.rs-1413 */     }
/* FP:hir.rs-1414 */ 
/* FP:hir.rs-1415 */     #[inline]
/* FP:hir.rs-1416 */     pub fn value_span(&self) -> Option<Span> {
/* FP:hir.rs-1417 */         AttributeExt::value_span(self)
/* FP:hir.rs-1418 */     }
/* FP:hir.rs-1419 */ 
/* FP:hir.rs-1420 */     #[inline]
/* FP:hir.rs-1421 */     pub fn ident(&self) -> Option<Ident> {
/* FP:hir.rs-1422 */         AttributeExt::ident(self)
/* FP:hir.rs-1423 */     }
/* FP:hir.rs-1424 */ 
/* FP:hir.rs-1425 */     #[inline]
/* FP:hir.rs-1426 */     pub fn path_matches(&self, name: &[Symbol]) -> bool {
/* FP:hir.rs-1427 */         AttributeExt::path_matches(self, name)
/* FP:hir.rs-1428 */     }
/* FP:hir.rs-1429 */ 
/* FP:hir.rs-1430 */     #[inline]
/* FP:hir.rs-1431 */     pub fn is_doc_comment(&self) -> bool {
/* FP:hir.rs-1432 */         AttributeExt::is_doc_comment(self)
/* FP:hir.rs-1433 */     }
/* FP:hir.rs-1434 */ 
/* FP:hir.rs-1435 */     #[inline]
/* FP:hir.rs-1436 */     pub fn has_name(&self, name: Symbol) -> bool {
/* FP:hir.rs-1437 */         AttributeExt::has_name(self, name)
/* FP:hir.rs-1438 */     }
/* FP:hir.rs-1439 */ 
/* FP:hir.rs-1440 */     #[inline]
/* FP:hir.rs-1441 */     pub fn has_any_name(&self, names: &[Symbol]) -> bool {
/* FP:hir.rs-1442 */         AttributeExt::has_any_name(self, names)
/* FP:hir.rs-1443 */     }
/* FP:hir.rs-1444 */ 
/* FP:hir.rs-1445 */     #[inline]
/* FP:hir.rs-1446 */     pub fn span(&self) -> Span {
/* FP:hir.rs-1447 */         AttributeExt::span(self)
/* FP:hir.rs-1448 */     }
/* FP:hir.rs-1449 */ 
/* FP:hir.rs-1450 */     #[inline]
/* FP:hir.rs-1451 */     pub fn is_word(&self) -> bool {
/* FP:hir.rs-1452 */         AttributeExt::is_word(self)
/* FP:hir.rs-1453 */     }
/* FP:hir.rs-1454 */ 
/* FP:hir.rs-1455 */     #[inline]
/* FP:hir.rs-1456 */     pub fn path(&self) -> SmallVec<[Symbol; 1]> {
/* FP:hir.rs-1457 */         AttributeExt::path(self)
/* FP:hir.rs-1458 */     }
/* FP:hir.rs-1459 */ 
/* FP:hir.rs-1460 */     #[inline]
/* FP:hir.rs-1461 */     pub fn ident_path(&self) -> Option<SmallVec<[Ident; 1]>> {
/* FP:hir.rs-1462 */         AttributeExt::ident_path(self)
/* FP:hir.rs-1463 */     }
/* FP:hir.rs-1464 */ 
/* FP:hir.rs-1465 */     #[inline]
/* FP:hir.rs-1466 */     pub fn doc_str(&self) -> Option<Symbol> {
/* FP:hir.rs-1467 */         AttributeExt::doc_str(self)
/* FP:hir.rs-1468 */     }
/* FP:hir.rs-1469 */ 
/* FP:hir.rs-1470 */     #[inline]
/* FP:hir.rs-1471 */     pub fn is_proc_macro_attr(&self) -> bool {
/* FP:hir.rs-1472 */         AttributeExt::is_proc_macro_attr(self)
/* FP:hir.rs-1473 */     }
/* FP:hir.rs-1474 */ 
/* FP:hir.rs-1475 */     #[inline]
/* FP:hir.rs-1476 */     pub fn doc_str_and_comment_kind(&self) -> Option<(Symbol, CommentKind)> {
/* FP:hir.rs-1477 */         AttributeExt::doc_str_and_comment_kind(self)
/* FP:hir.rs-1478 */     }
/* FP:hir.rs-1479 */ }
/* FP:hir.rs-1480 */ 
/* FP:hir.rs-1481 */ /// Attributes owned by a HIR owner.
/* FP:hir.rs-1482 */ #[derive(Debug)]
/* FP:hir.rs-1483 */ pub struct AttributeMap<'tcx> {
/* FP:hir.rs-1484 */     pub map: SortedMap<ItemLocalId, &'tcx [Attribute]>,
/* FP:hir.rs-1485 */     /// Preprocessed `#[define_opaque]` attribute.
/* FP:hir.rs-1486 */     pub define_opaque: Option<&'tcx [(Span, LocalDefId)]>,
/* FP:hir.rs-1487 */     // Only present when the crate hash is needed.
/* FP:hir.rs-1488 */     pub opt_hash: Option<Fingerprint>,
/* FP:hir.rs-1489 */ }
/* FP:hir.rs-1490 */ 
/* FP:hir.rs-1491 */ impl<'tcx> AttributeMap<'tcx> {
/* FP:hir.rs-1492 */     pub const EMPTY: &'static AttributeMap<'static> = &AttributeMap {
/* FP:hir.rs-1493 */         map: SortedMap::new(),
/* FP:hir.rs-1494 */         opt_hash: Some(Fingerprint::ZERO),
/* FP:hir.rs-1495 */         define_opaque: None,
/* FP:hir.rs-1496 */     };
/* FP:hir.rs-1497 */ 
/* FP:hir.rs-1498 */     #[inline]
/* FP:hir.rs-1499 */     pub fn get(&self, id: ItemLocalId) -> &'tcx [Attribute] {
/* FP:hir.rs-1500 */         self.map.get(&id).copied().unwrap_or(&[])
/* FP:hir.rs-1501 */     }
/* FP:hir.rs-1502 */ }
/* FP:hir.rs-1503 */ 
/* FP:hir.rs-1504 */ /// Map of all HIR nodes inside the current owner.
/* FP:hir.rs-1505 */ /// These nodes are mapped by `ItemLocalId` alongside the index of their parent node.
/* FP:hir.rs-1506 */ /// The HIR tree, including bodies, is pre-hashed.
/* FP:hir.rs-1507 */ pub struct OwnerNodes<'tcx> {
/* FP:hir.rs-1508 */     /// Pre-computed hash of the full HIR. Used in the crate hash. Only present
/* FP:hir.rs-1509 */     /// when incr. comp. is enabled.
/* FP:hir.rs-1510 */     pub opt_hash_including_bodies: Option<Fingerprint>,
/* FP:hir.rs-1511 */     /// Full HIR for the current owner.
/* FP:hir.rs-1512 */     // The zeroth node's parent should never be accessed: the owner's parent is computed by the
/* FP:hir.rs-1513 */     // hir_owner_parent query. It is set to `ItemLocalId::INVALID` to force an ICE if accidentally
/* FP:hir.rs-1514 */     // used.
/* FP:hir.rs-1515 */     pub nodes: IndexVec<ItemLocalId, ParentedNode<'tcx>>,
/* FP:hir.rs-1516 */     /// Content of local bodies.
/* FP:hir.rs-1517 */     pub bodies: SortedMap<ItemLocalId, &'tcx Body<'tcx>>,
/* FP:hir.rs-1518 */ }
/* FP:hir.rs-1519 */ 
/* FP:hir.rs-1520 */ impl<'tcx> OwnerNodes<'tcx> {
/* FP:hir.rs-1521 */     pub fn node(&self) -> OwnerNode<'tcx> {
/* FP:hir.rs-1522 */         // Indexing must ensure it is an OwnerNode.
/* FP:hir.rs-1523 */         self.nodes[ItemLocalId::ZERO].node.as_owner().unwrap()
/* FP:hir.rs-1524 */     }
/* FP:hir.rs-1525 */ }
/* FP:hir.rs-1526 */ 
/* FP:hir.rs-1527 */ impl fmt::Debug for OwnerNodes<'_> {
/* FP:hir.rs-1528 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-1529 */         f.debug_struct("OwnerNodes")
/* FP:hir.rs-1530 */             // Do not print all the pointers to all the nodes, as it would be unreadable.
/* FP:hir.rs-1531 */             .field("node", &self.nodes[ItemLocalId::ZERO])
/* FP:hir.rs-1532 */             .field(
/* FP:hir.rs-1533 */                 "parents",
/* FP:hir.rs-1534 */                 &fmt::from_fn(|f| {
/* FP:hir.rs-1535 */                     f.debug_list()
/* FP:hir.rs-1536 */                         .entries(self.nodes.iter_enumerated().map(|(id, parented_node)| {
/* FP:hir.rs-1537 */                             fmt::from_fn(move |f| write!(f, "({id:?}, {:?})", parented_node.parent))
/* FP:hir.rs-1538 */                         }))
/* FP:hir.rs-1539 */                         .finish()
/* FP:hir.rs-1540 */                 }),
/* FP:hir.rs-1541 */             )
/* FP:hir.rs-1542 */             .field("bodies", &self.bodies)
/* FP:hir.rs-1543 */             .field("opt_hash_including_bodies", &self.opt_hash_including_bodies)
/* FP:hir.rs-1544 */             .finish()
/* FP:hir.rs-1545 */     }
/* FP:hir.rs-1546 */ }
/* FP:hir.rs-1547 */ 
/* FP:hir.rs-1548 */ /// Full information resulting from lowering an AST node.
/* FP:hir.rs-1549 */ #[derive(Debug, HashStable_Generic)]
/* FP:hir.rs-1550 */ pub struct OwnerInfo<'hir> {
/* FP:hir.rs-1551 */     /// Contents of the HIR.
/* FP:hir.rs-1552 */     pub nodes: OwnerNodes<'hir>,
/* FP:hir.rs-1553 */     /// Map from each nested owner to its parent's local id.
/* FP:hir.rs-1554 */     pub parenting: LocalDefIdMap<ItemLocalId>,
/* FP:hir.rs-1555 */     /// Collected attributes of the HIR nodes.
/* FP:hir.rs-1556 */     pub attrs: AttributeMap<'hir>,
/* FP:hir.rs-1557 */     /// Map indicating what traits are in scope for places where this
/* FP:hir.rs-1558 */     /// is relevant; generated by resolve.
/* FP:hir.rs-1559 */     pub trait_map: ItemLocalMap<Box<[TraitCandidate]>>,
/* FP:hir.rs-1560 */ 
/* FP:hir.rs-1561 */     /// Lints delayed during ast lowering to be emitted
/* FP:hir.rs-1562 */     /// after hir has completely built
/* FP:hir.rs-1563 */     pub delayed_lints: DelayedLints,
/* FP:hir.rs-1564 */ }
/* FP:hir.rs-1565 */ 
/* FP:hir.rs-1566 */ impl<'tcx> OwnerInfo<'tcx> {
/* FP:hir.rs-1567 */     #[inline]
/* FP:hir.rs-1568 */     pub fn node(&self) -> OwnerNode<'tcx> {
/* FP:hir.rs-1569 */         self.nodes.node()
/* FP:hir.rs-1570 */     }
/* FP:hir.rs-1571 */ }
/* FP:hir.rs-1572 */ 
/* FP:hir.rs-1573 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-1574 */ pub enum MaybeOwner<'tcx> {
/* FP:hir.rs-1575 */     Owner(&'tcx OwnerInfo<'tcx>),
/* FP:hir.rs-1576 */     NonOwner(HirId),
/* FP:hir.rs-1577 */     /// Used as a placeholder for unused LocalDefId.
/* FP:hir.rs-1578 */     Phantom,
/* FP:hir.rs-1579 */ }
/* FP:hir.rs-1580 */ 
/* FP:hir.rs-1581 */ impl<'tcx> MaybeOwner<'tcx> {
/* FP:hir.rs-1582 */     pub fn as_owner(self) -> Option<&'tcx OwnerInfo<'tcx>> {
/* FP:hir.rs-1583 */         match self {
/* FP:hir.rs-1584 */             MaybeOwner::Owner(i) => Some(i),
/* FP:hir.rs-1585 */             MaybeOwner::NonOwner(_) | MaybeOwner::Phantom => None,
/* FP:hir.rs-1586 */         }
/* FP:hir.rs-1587 */     }
/* FP:hir.rs-1588 */ 
/* FP:hir.rs-1589 */     pub fn unwrap(self) -> &'tcx OwnerInfo<'tcx> {
/* FP:hir.rs-1590 */         self.as_owner().unwrap_or_else(|| panic!("Not a HIR owner"))
/* FP:hir.rs-1591 */     }
/* FP:hir.rs-1592 */ }
/* FP:hir.rs-1593 */ 
/* FP:hir.rs-1594 */ /// The top-level data structure that stores the entire contents of
/* FP:hir.rs-1595 */ /// the crate currently being compiled.
/* FP:hir.rs-1596 */ ///
/* FP:hir.rs-1597 */ /// For more details, see the [rustc dev guide].
/* FP:hir.rs-1598 */ ///
/* FP:hir.rs-1599 */ /// [rustc dev guide]: https://rustc-dev-guide.rust-lang.org/hir.html
/* FP:hir.rs-1600 */ #[derive(Debug)]
/* FP:hir.rs-1601 */ pub struct Crate<'hir> {
/* FP:hir.rs-1602 */     pub owners: IndexVec<LocalDefId, MaybeOwner<'hir>>,
/* FP:hir.rs-1603 */     // Only present when incr. comp. is enabled.
/* FP:hir.rs-1604 */     pub opt_hir_hash: Option<Fingerprint>,
/* FP:hir.rs-1605 */ }
/* FP:hir.rs-1606 */ 
/* FP:hir.rs-1607 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1608 */ pub struct Closure<'hir> {
/* FP:hir.rs-1609 */     pub def_id: LocalDefId,
/* FP:hir.rs-1610 */     pub binder: ClosureBinder,
/* FP:hir.rs-1611 */     pub constness: Constness,
/* FP:hir.rs-1612 */     pub capture_clause: CaptureBy,
/* FP:hir.rs-1613 */     pub bound_generic_params: &'hir [GenericParam<'hir>],
/* FP:hir.rs-1614 */     pub fn_decl: &'hir FnDecl<'hir>,
/* FP:hir.rs-1615 */     pub body: BodyId,
/* FP:hir.rs-1616 */     /// The span of the declaration block: 'move |...| -> ...'
/* FP:hir.rs-1617 */     pub fn_decl_span: Span,
/* FP:hir.rs-1618 */     /// The span of the argument block `|...|`
/* FP:hir.rs-1619 */     pub fn_arg_span: Option<Span>,
/* FP:hir.rs-1620 */     pub kind: ClosureKind,
/* FP:hir.rs-1621 */ }
/* FP:hir.rs-1622 */ 
/* FP:hir.rs-1623 */ #[derive(Clone, PartialEq, Eq, Debug, Copy, Hash, HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-1624 */ pub enum ClosureKind {
/* FP:hir.rs-1625 */     /// This is a plain closure expression.
/* FP:hir.rs-1626 */     Closure,
/* FP:hir.rs-1627 */     /// This is a coroutine expression -- i.e. a closure expression in which
/* FP:hir.rs-1628 */     /// we've found a `yield`. These can arise either from "plain" coroutine
/* FP:hir.rs-1629 */     ///  usage (e.g. `let x = || { yield (); }`) or from a desugared expression
/* FP:hir.rs-1630 */     /// (e.g. `async` and `gen` blocks).
/* FP:hir.rs-1631 */     Coroutine(CoroutineKind),
/* FP:hir.rs-1632 */     /// This is a coroutine-closure, which is a special sugared closure that
/* FP:hir.rs-1633 */     /// returns one of the sugared coroutine (`async`/`gen`/`async gen`). It
/* FP:hir.rs-1634 */     /// additionally allows capturing the coroutine's upvars by ref, and therefore
/* FP:hir.rs-1635 */     /// needs to be specially treated during analysis and borrowck.
/* FP:hir.rs-1636 */     CoroutineClosure(CoroutineDesugaring),
/* FP:hir.rs-1637 */ }
/* FP:hir.rs-1638 */ 
/* FP:hir.rs-1639 */ /// A block of statements `{ .. }`, which may have a label (in this case the
/* FP:hir.rs-1640 */ /// `targeted_by_break` field will be `true`) and may be `unsafe` by means of
/* FP:hir.rs-1641 */ /// the `rules` being anything but `DefaultBlock`.
/* FP:hir.rs-1642 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1643 */ pub struct Block<'hir> {
/* FP:hir.rs-1644 */     /// Statements in a block.
/* FP:hir.rs-1645 */     pub stmts: &'hir [Stmt<'hir>],
/* FP:hir.rs-1646 */     /// An expression at the end of the block
/* FP:hir.rs-1647 */     /// without a semicolon, if any.
/* FP:hir.rs-1648 */     pub expr: Option<&'hir Expr<'hir>>,
/* FP:hir.rs-1649 */     #[stable_hasher(ignore)]
/* FP:hir.rs-1650 */     pub hir_id: HirId,
/* FP:hir.rs-1651 */     /// Distinguishes between `unsafe { ... }` and `{ ... }`.
/* FP:hir.rs-1652 */     pub rules: BlockCheckMode,
/* FP:hir.rs-1653 */     /// The span includes the curly braces `{` and `}` around the block.
/* FP:hir.rs-1654 */     pub span: Span,
/* FP:hir.rs-1655 */     /// If true, then there may exist `break 'a` values that aim to
/* FP:hir.rs-1656 */     /// break out of this block early.
/* FP:hir.rs-1657 */     /// Used by `'label: {}` blocks and by `try {}` blocks.
/* FP:hir.rs-1658 */     pub targeted_by_break: bool,
/* FP:hir.rs-1659 */ }
/* FP:hir.rs-1660 */ 
/* FP:hir.rs-1661 */ impl<'hir> Block<'hir> {
/* FP:hir.rs-1662 */     pub fn innermost_block(&self) -> &Block<'hir> {
/* FP:hir.rs-1663 */         let mut block = self;
/* FP:hir.rs-1664 */         while let Some(Expr { kind: ExprKind::Block(inner_block, _), .. }) = block.expr {
/* FP:hir.rs-1665 */             block = inner_block;
/* FP:hir.rs-1666 */         }
/* FP:hir.rs-1667 */         block
/* FP:hir.rs-1668 */     }
/* FP:hir.rs-1669 */ }
/* FP:hir.rs-1670 */ 
/* FP:hir.rs-1671 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1672 */ pub struct TyPat<'hir> {
/* FP:hir.rs-1673 */     #[stable_hasher(ignore)]
/* FP:hir.rs-1674 */     pub hir_id: HirId,
/* FP:hir.rs-1675 */     pub kind: TyPatKind<'hir>,
/* FP:hir.rs-1676 */     pub span: Span,
/* FP:hir.rs-1677 */ }
/* FP:hir.rs-1678 */ 
/* FP:hir.rs-1679 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1680 */ pub struct Pat<'hir> {
/* FP:hir.rs-1681 */     #[stable_hasher(ignore)]
/* FP:hir.rs-1682 */     pub hir_id: HirId,
/* FP:hir.rs-1683 */     pub kind: PatKind<'hir>,
/* FP:hir.rs-1684 */     pub span: Span,
/* FP:hir.rs-1685 */     /// Whether to use default binding modes.
/* FP:hir.rs-1686 */     /// At present, this is false only for destructuring assignment.
/* FP:hir.rs-1687 */     pub default_binding_modes: bool,
/* FP:hir.rs-1688 */ }
/* FP:hir.rs-1689 */ 
/* FP:hir.rs-1690 */ impl<'hir> Pat<'hir> {
/* FP:hir.rs-1691 */     fn walk_short_(&self, it: &mut impl FnMut(&Pat<'hir>) -> bool) -> bool {
/* FP:hir.rs-1692 */         if !it(self) {
/* FP:hir.rs-1693 */             return false;
/* FP:hir.rs-1694 */         }
/* FP:hir.rs-1695 */ 
/* FP:hir.rs-1696 */         use PatKind::*;
/* FP:hir.rs-1697 */         match self.kind {
/* FP:hir.rs-1698 */             Missing => unreachable!(),
/* FP:hir.rs-1699 */             Wild | Never | Expr(_) | Range(..) | Binding(.., None) | Err(_) => true,
/* FP:hir.rs-1700 */             Box(s) | Deref(s) | Ref(s, _) | Binding(.., Some(s)) | Guard(s, _) => s.walk_short_(it),
/* FP:hir.rs-1701 */             Struct(_, fields, _) => fields.iter().all(|field| field.pat.walk_short_(it)),
/* FP:hir.rs-1702 */             TupleStruct(_, s, _) | Tuple(s, _) | Or(s) => s.iter().all(|p| p.walk_short_(it)),
/* FP:hir.rs-1703 */             Slice(before, slice, after) => {
/* FP:hir.rs-1704 */                 before.iter().chain(slice).chain(after.iter()).all(|p| p.walk_short_(it))
/* FP:hir.rs-1705 */             }
/* FP:hir.rs-1706 */         }
/* FP:hir.rs-1707 */     }
/* FP:hir.rs-1708 */ 
/* FP:hir.rs-1709 */     /// Walk the pattern in left-to-right order,
/* FP:hir.rs-1710 */     /// short circuiting (with `.all(..)`) if `false` is returned.
/* FP:hir.rs-1711 */     ///
/* FP:hir.rs-1712 */     /// Note that when visiting e.g. `Tuple(ps)`,
/* FP:hir.rs-1713 */     /// if visiting `ps[0]` returns `false`,
/* FP:hir.rs-1714 */     /// then `ps[1]` will not be visited.
/* FP:hir.rs-1715 */     pub fn walk_short(&self, mut it: impl FnMut(&Pat<'hir>) -> bool) -> bool {
/* FP:hir.rs-1716 */         self.walk_short_(&mut it)
/* FP:hir.rs-1717 */     }
/* FP:hir.rs-1718 */ 
/* FP:hir.rs-1719 */     fn walk_(&self, it: &mut impl FnMut(&Pat<'hir>) -> bool) {
/* FP:hir.rs-1720 */         if !it(self) {
/* FP:hir.rs-1721 */             return;
/* FP:hir.rs-1722 */         }
/* FP:hir.rs-1723 */ 
/* FP:hir.rs-1724 */         use PatKind::*;
/* FP:hir.rs-1725 */         match self.kind {
/* FP:hir.rs-1726 */             Missing | Wild | Never | Expr(_) | Range(..) | Binding(.., None) | Err(_) => {}
/* FP:hir.rs-1727 */             Box(s) | Deref(s) | Ref(s, _) | Binding(.., Some(s)) | Guard(s, _) => s.walk_(it),
/* FP:hir.rs-1728 */             Struct(_, fields, _) => fields.iter().for_each(|field| field.pat.walk_(it)),
/* FP:hir.rs-1729 */             TupleStruct(_, s, _) | Tuple(s, _) | Or(s) => s.iter().for_each(|p| p.walk_(it)),
/* FP:hir.rs-1730 */             Slice(before, slice, after) => {
/* FP:hir.rs-1731 */                 before.iter().chain(slice).chain(after.iter()).for_each(|p| p.walk_(it))
/* FP:hir.rs-1732 */             }
/* FP:hir.rs-1733 */         }
/* FP:hir.rs-1734 */     }
/* FP:hir.rs-1735 */ 
/* FP:hir.rs-1736 */     /// Walk the pattern in left-to-right order.
/* FP:hir.rs-1737 */     ///
/* FP:hir.rs-1738 */     /// If `it(pat)` returns `false`, the children are not visited.
/* FP:hir.rs-1739 */     pub fn walk(&self, mut it: impl FnMut(&Pat<'hir>) -> bool) {
/* FP:hir.rs-1740 */         self.walk_(&mut it)
/* FP:hir.rs-1741 */     }
/* FP:hir.rs-1742 */ 
/* FP:hir.rs-1743 */     /// Walk the pattern in left-to-right order.
/* FP:hir.rs-1744 */     ///
/* FP:hir.rs-1745 */     /// If you always want to recurse, prefer this method over `walk`.
/* FP:hir.rs-1746 */     pub fn walk_always(&self, mut it: impl FnMut(&Pat<'_>)) {
/* FP:hir.rs-1747 */         self.walk(|p| {
/* FP:hir.rs-1748 */             it(p);
/* FP:hir.rs-1749 */             true
/* FP:hir.rs-1750 */         })
/* FP:hir.rs-1751 */     }
/* FP:hir.rs-1752 */ 
/* FP:hir.rs-1753 */     /// Whether this a never pattern.
/* FP:hir.rs-1754 */     pub fn is_never_pattern(&self) -> bool {
/* FP:hir.rs-1755 */         let mut is_never_pattern = false;
/* FP:hir.rs-1756 */         self.walk(|pat| match &pat.kind {
/* FP:hir.rs-1757 */             PatKind::Never => {
/* FP:hir.rs-1758 */                 is_never_pattern = true;
/* FP:hir.rs-1759 */                 false
/* FP:hir.rs-1760 */             }
/* FP:hir.rs-1761 */             PatKind::Or(s) => {
/* FP:hir.rs-1762 */                 is_never_pattern = s.iter().all(|p| p.is_never_pattern());
/* FP:hir.rs-1763 */                 false
/* FP:hir.rs-1764 */             }
/* FP:hir.rs-1765 */             _ => true,
/* FP:hir.rs-1766 */         });
/* FP:hir.rs-1767 */         is_never_pattern
/* FP:hir.rs-1768 */     }
/* FP:hir.rs-1769 */ }
/* FP:hir.rs-1770 */ 
/* FP:hir.rs-1771 */ /// A single field in a struct pattern.
/* FP:hir.rs-1772 */ ///
/* FP:hir.rs-1773 */ /// Patterns like the fields of Foo `{ x, ref y, ref mut z }`
/* FP:hir.rs-1774 */ /// are treated the same as` x: x, y: ref y, z: ref mut z`,
/* FP:hir.rs-1775 */ /// except `is_shorthand` is true.
/* FP:hir.rs-1776 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1777 */ pub struct PatField<'hir> {
/* FP:hir.rs-1778 */     #[stable_hasher(ignore)]
/* FP:hir.rs-1779 */     pub hir_id: HirId,
/* FP:hir.rs-1780 */     /// The identifier for the field.
/* FP:hir.rs-1781 */     pub ident: Ident,
/* FP:hir.rs-1782 */     /// The pattern the field is destructured to.
/* FP:hir.rs-1783 */     pub pat: &'hir Pat<'hir>,
/* FP:hir.rs-1784 */     pub is_shorthand: bool,
/* FP:hir.rs-1785 */     pub span: Span,
/* FP:hir.rs-1786 */ }
/* FP:hir.rs-1787 */ 
/* FP:hir.rs-1788 */ #[derive(Copy, Clone, PartialEq, Debug, HashStable_Generic, Hash, Eq, Encodable, Decodable)]
/* FP:hir.rs-1789 */ pub enum RangeEnd {
/* FP:hir.rs-1790 */     Included,
/* FP:hir.rs-1791 */     Excluded,
/* FP:hir.rs-1792 */ }
/* FP:hir.rs-1793 */ 
/* FP:hir.rs-1794 */ impl fmt::Display for RangeEnd {
/* FP:hir.rs-1795 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-1796 */         f.write_str(match self {
/* FP:hir.rs-1797 */             RangeEnd::Included => "..=",
/* FP:hir.rs-1798 */             RangeEnd::Excluded => "..",
/* FP:hir.rs-1799 */         })
/* FP:hir.rs-1800 */     }
/* FP:hir.rs-1801 */ }
/* FP:hir.rs-1802 */ 
/* FP:hir.rs-1803 */ // Equivalent to `Option<usize>`. That type takes up 16 bytes on 64-bit, but
/* FP:hir.rs-1804 */ // this type only takes up 4 bytes, at the cost of being restricted to a
/* FP:hir.rs-1805 */ // maximum value of `u32::MAX - 1`. In practice, this is more than enough.
/* FP:hir.rs-1806 */ #[derive(Clone, Copy, PartialEq, Eq, Hash, HashStable_Generic)]
/* FP:hir.rs-1807 */ pub struct DotDotPos(u32);
/* FP:hir.rs-1808 */ 
/* FP:hir.rs-1809 */ impl DotDotPos {
/* FP:hir.rs-1810 */     /// Panics if n >= u32::MAX.
/* FP:hir.rs-1811 */     pub fn new(n: Option<usize>) -> Self {
/* FP:hir.rs-1812 */         match n {
/* FP:hir.rs-1813 */             Some(n) => {
/* FP:hir.rs-1814 */                 assert!(n < u32::MAX as usize);
/* FP:hir.rs-1815 */                 Self(n as u32)
/* FP:hir.rs-1816 */             }
/* FP:hir.rs-1817 */             None => Self(u32::MAX),
/* FP:hir.rs-1818 */         }
/* FP:hir.rs-1819 */     }
/* FP:hir.rs-1820 */ 
/* FP:hir.rs-1821 */     pub fn as_opt_usize(&self) -> Option<usize> {
/* FP:hir.rs-1822 */         if self.0 == u32::MAX { None } else { Some(self.0 as usize) }
/* FP:hir.rs-1823 */     }
/* FP:hir.rs-1824 */ }
/* FP:hir.rs-1825 */ 
/* FP:hir.rs-1826 */ impl fmt::Debug for DotDotPos {
/* FP:hir.rs-1827 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-1828 */         self.as_opt_usize().fmt(f)
/* FP:hir.rs-1829 */     }
/* FP:hir.rs-1830 */ }
/* FP:hir.rs-1831 */ 
/* FP:hir.rs-1832 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1833 */ pub struct PatExpr<'hir> {
/* FP:hir.rs-1834 */     #[stable_hasher(ignore)]
/* FP:hir.rs-1835 */     pub hir_id: HirId,
/* FP:hir.rs-1836 */     pub span: Span,
/* FP:hir.rs-1837 */     pub kind: PatExprKind<'hir>,
/* FP:hir.rs-1838 */ }
/* FP:hir.rs-1839 */ 
/* FP:hir.rs-1840 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1841 */ pub enum PatExprKind<'hir> {
/* FP:hir.rs-1842 */     Lit {
/* FP:hir.rs-1843 */         lit: Lit,
/* FP:hir.rs-1844 */         // FIXME: move this into `Lit` and handle negated literal expressions
/* FP:hir.rs-1845 */         // once instead of matching on unop neg expressions everywhere.
/* FP:hir.rs-1846 */         negated: bool,
/* FP:hir.rs-1847 */     },
/* FP:hir.rs-1848 */     ConstBlock(ConstBlock),
/* FP:hir.rs-1849 */     /// A path pattern for a unit struct/variant or a (maybe-associated) constant.
/* FP:hir.rs-1850 */     Path(QPath<'hir>),
/* FP:hir.rs-1851 */ }
/* FP:hir.rs-1852 */ 
/* FP:hir.rs-1853 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1854 */ pub enum TyPatKind<'hir> {
/* FP:hir.rs-1855 */     /// A range pattern (e.g., `1..=2` or `1..2`).
/* FP:hir.rs-1856 */     Range(&'hir ConstArg<'hir>, &'hir ConstArg<'hir>),
/* FP:hir.rs-1857 */ 
/* FP:hir.rs-1858 */     /// A list of patterns where only one needs to be satisfied
/* FP:hir.rs-1859 */     Or(&'hir [TyPat<'hir>]),
/* FP:hir.rs-1860 */ 
/* FP:hir.rs-1861 */     /// A placeholder for a pattern that wasn't well formed in some way.
/* FP:hir.rs-1862 */     Err(ErrorGuaranteed),
/* FP:hir.rs-1863 */ }
/* FP:hir.rs-1864 */ 
/* FP:hir.rs-1865 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1866 */ pub enum PatKind<'hir> {
/* FP:hir.rs-1867 */     /// A missing pattern, e.g. for an anonymous param in a bare fn like `fn f(u32)`.
/* FP:hir.rs-1868 */     Missing,
/* FP:hir.rs-1869 */ 
/* FP:hir.rs-1870 */     /// Represents a wildcard pattern (i.e., `_`).
/* FP:hir.rs-1871 */     Wild,
/* FP:hir.rs-1872 */ 
/* FP:hir.rs-1873 */     /// A fresh binding `ref mut binding @ OPT_SUBPATTERN`.
/* FP:hir.rs-1874 */     /// The `HirId` is the canonical ID for the variable being bound,
/* FP:hir.rs-1875 */     /// (e.g., in `Ok(x) | Err(x)`, both `x` use the same canonical ID),
/* FP:hir.rs-1876 */     /// which is the pattern ID of the first `x`.
/* FP:hir.rs-1877 */     ///
/* FP:hir.rs-1878 */     /// The `BindingMode` is what's provided by the user, before match
/* FP:hir.rs-1879 */     /// ergonomics are applied. For the binding mode actually in use,
/* FP:hir.rs-1880 */     /// see [`TypeckResults::extract_binding_mode`].
/* FP:hir.rs-1881 */     ///
/* FP:hir.rs-1882 */     /// [`TypeckResults::extract_binding_mode`]: ../../rustc_middle/ty/struct.TypeckResults.html#method.extract_binding_mode
/* FP:hir.rs-1883 */     Binding(BindingMode, HirId, Ident, Option<&'hir Pat<'hir>>),
/* FP:hir.rs-1884 */ 
/* FP:hir.rs-1885 */     /// A struct or struct variant pattern (e.g., `Variant {x, y, ..}`).
/* FP:hir.rs-1886 */     /// The `Option` contains the span of a possible `..`.
/* FP:hir.rs-1887 */     Struct(QPath<'hir>, &'hir [PatField<'hir>], Option<Span>),
/* FP:hir.rs-1888 */ 
/* FP:hir.rs-1889 */     /// A tuple struct/variant pattern `Variant(x, y, .., z)`.
/* FP:hir.rs-1890 */     /// If the `..` pattern fragment is present, then `DotDotPos` denotes its position.
/* FP:hir.rs-1891 */     /// `0 <= position <= subpats.len()`
/* FP:hir.rs-1892 */     TupleStruct(QPath<'hir>, &'hir [Pat<'hir>], DotDotPos),
/* FP:hir.rs-1893 */ 
/* FP:hir.rs-1894 */     /// An or-pattern `A | B | C`.
/* FP:hir.rs-1895 */     /// Invariant: `pats.len() >= 2`.
/* FP:hir.rs-1896 */     Or(&'hir [Pat<'hir>]),
/* FP:hir.rs-1897 */ 
/* FP:hir.rs-1898 */     /// A never pattern `!`.
/* FP:hir.rs-1899 */     Never,
/* FP:hir.rs-1900 */ 
/* FP:hir.rs-1901 */     /// A tuple pattern (e.g., `(a, b)`).
/* FP:hir.rs-1902 */     /// If the `..` pattern fragment is present, then `DotDotPos` denotes its position.
/* FP:hir.rs-1903 */     /// `0 <= position <= subpats.len()`
/* FP:hir.rs-1904 */     Tuple(&'hir [Pat<'hir>], DotDotPos),
/* FP:hir.rs-1905 */ 
/* FP:hir.rs-1906 */     /// A `box` pattern.
/* FP:hir.rs-1907 */     Box(&'hir Pat<'hir>),
/* FP:hir.rs-1908 */ 
/* FP:hir.rs-1909 */     /// A `deref` pattern (currently `deref!()` macro-based syntax).
/* FP:hir.rs-1910 */     Deref(&'hir Pat<'hir>),
/* FP:hir.rs-1911 */ 
/* FP:hir.rs-1912 */     /// A reference pattern (e.g., `&mut (a, b)`).
/* FP:hir.rs-1913 */     Ref(&'hir Pat<'hir>, Mutability),
/* FP:hir.rs-1914 */ 
/* FP:hir.rs-1915 */     /// A literal, const block or path.
/* FP:hir.rs-1916 */     Expr(&'hir PatExpr<'hir>),
/* FP:hir.rs-1917 */ 
/* FP:hir.rs-1918 */     /// A guard pattern (e.g., `x if guard(x)`).
/* FP:hir.rs-1919 */     Guard(&'hir Pat<'hir>, &'hir Expr<'hir>),
/* FP:hir.rs-1920 */ 
/* FP:hir.rs-1921 */     /// A range pattern (e.g., `1..=2` or `1..2`).
/* FP:hir.rs-1922 */     Range(Option<&'hir PatExpr<'hir>>, Option<&'hir PatExpr<'hir>>, RangeEnd),
/* FP:hir.rs-1923 */ 
/* FP:hir.rs-1924 */     /// A slice pattern, `[before_0, ..., before_n, (slice, after_0, ..., after_n)?]`.
/* FP:hir.rs-1925 */     ///
/* FP:hir.rs-1926 */     /// Here, `slice` is lowered from the syntax `($binding_mode $ident @)? ..`.
/* FP:hir.rs-1927 */     /// If `slice` exists, then `after` can be non-empty.
/* FP:hir.rs-1928 */     ///
/* FP:hir.rs-1929 */     /// The representation for e.g., `[a, b, .., c, d]` is:
/* FP:hir.rs-1930 */     /// ```ignore (illustrative)
/* FP:hir.rs-1931 */     /// PatKind::Slice([Binding(a), Binding(b)], Some(Wild), [Binding(c), Binding(d)])
/* FP:hir.rs-1932 */     /// ```
/* FP:hir.rs-1933 */     Slice(&'hir [Pat<'hir>], Option<&'hir Pat<'hir>>, &'hir [Pat<'hir>]),
/* FP:hir.rs-1934 */ 
/* FP:hir.rs-1935 */     /// A placeholder for a pattern that wasn't well formed in some way.
/* FP:hir.rs-1936 */     Err(ErrorGuaranteed),
/* FP:hir.rs-1937 */ }
/* FP:hir.rs-1938 */ 
/* FP:hir.rs-1939 */ /// A statement.
/* FP:hir.rs-1940 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1941 */ pub struct Stmt<'hir> {
/* FP:hir.rs-1942 */     #[stable_hasher(ignore)]
/* FP:hir.rs-1943 */     pub hir_id: HirId,
/* FP:hir.rs-1944 */     pub kind: StmtKind<'hir>,
/* FP:hir.rs-1945 */     pub span: Span,
/* FP:hir.rs-1946 */ }
/* FP:hir.rs-1947 */ 
/* FP:hir.rs-1948 */ /// The contents of a statement.
/* FP:hir.rs-1949 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1950 */ pub enum StmtKind<'hir> {
/* FP:hir.rs-1951 */     /// A local (`let`) binding.
/* FP:hir.rs-1952 */     Let(&'hir LetStmt<'hir>),
/* FP:hir.rs-1953 */ 
/* FP:hir.rs-1954 */     /// An item binding.
/* FP:hir.rs-1955 */     Item(ItemId),
/* FP:hir.rs-1956 */ 
/* FP:hir.rs-1957 */     /// An expression without a trailing semi-colon (must have unit type).
/* FP:hir.rs-1958 */     Expr(&'hir Expr<'hir>),
/* FP:hir.rs-1959 */ 
/* FP:hir.rs-1960 */     /// An expression with a trailing semi-colon (may have any type).
/* FP:hir.rs-1961 */     Semi(&'hir Expr<'hir>),
/* FP:hir.rs-1962 */ }
/* FP:hir.rs-1963 */ 
/* FP:hir.rs-1964 */ /// Represents a `let` statement (i.e., `let <pat>:<ty> = <init>;`).
/* FP:hir.rs-1965 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1966 */ pub struct LetStmt<'hir> {
/* FP:hir.rs-1967 */     /// Span of `super` in `super let`.
/* FP:hir.rs-1968 */     pub super_: Option<Span>,
/* FP:hir.rs-1969 */     pub pat: &'hir Pat<'hir>,
/* FP:hir.rs-1970 */     /// Type annotation, if any (otherwise the type will be inferred).
/* FP:hir.rs-1971 */     pub ty: Option<&'hir Ty<'hir>>,
/* FP:hir.rs-1972 */     /// Initializer expression to set the value, if any.
/* FP:hir.rs-1973 */     pub init: Option<&'hir Expr<'hir>>,
/* FP:hir.rs-1974 */     /// Else block for a `let...else` binding.
/* FP:hir.rs-1975 */     pub els: Option<&'hir Block<'hir>>,
/* FP:hir.rs-1976 */     #[stable_hasher(ignore)]
/* FP:hir.rs-1977 */     pub hir_id: HirId,
/* FP:hir.rs-1978 */     pub span: Span,
/* FP:hir.rs-1979 */     /// Can be `ForLoopDesugar` if the `let` statement is part of a `for` loop
/* FP:hir.rs-1980 */     /// desugaring, or `AssignDesugar` if it is the result of a complex
/* FP:hir.rs-1981 */     /// assignment desugaring. Otherwise will be `Normal`.
/* FP:hir.rs-1982 */     pub source: LocalSource,
/* FP:hir.rs-1983 */ }
/* FP:hir.rs-1984 */ 
/* FP:hir.rs-1985 */ /// Represents a single arm of a `match` expression, e.g.
/* FP:hir.rs-1986 */ /// `<pat> (if <guard>) => <body>`.
/* FP:hir.rs-1987 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-1988 */ pub struct Arm<'hir> {
/* FP:hir.rs-1989 */     #[stable_hasher(ignore)]
/* FP:hir.rs-1990 */     pub hir_id: HirId,
/* FP:hir.rs-1991 */     pub span: Span,
/* FP:hir.rs-1992 */     /// If this pattern and the optional guard matches, then `body` is evaluated.
/* FP:hir.rs-1993 */     pub pat: &'hir Pat<'hir>,
/* FP:hir.rs-1994 */     /// Optional guard clause.
/* FP:hir.rs-1995 */     pub guard: Option<&'hir Expr<'hir>>,
/* FP:hir.rs-1996 */     /// The expression the arm evaluates to if this arm matches.
/* FP:hir.rs-1997 */     pub body: &'hir Expr<'hir>,
/* FP:hir.rs-1998 */ }
/* FP:hir.rs-1999 */ 
/* FP:hir.rs-2000 */ /// Represents a `let <pat>[: <ty>] = <expr>` expression (not a [`LetStmt`]), occurring in an `if-let`
/* FP:hir.rs-2001 */ /// or `let-else`, evaluating to a boolean. Typically the pattern is refutable.
/* FP:hir.rs-2002 */ ///
/* FP:hir.rs-2003 */ /// In an `if let`, imagine it as `if (let <pat> = <expr>) { ... }`; in a let-else, it is part of
/* FP:hir.rs-2004 */ /// the desugaring to if-let. Only let-else supports the type annotation at present.
/* FP:hir.rs-2005 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-2006 */ pub struct LetExpr<'hir> {
/* FP:hir.rs-2007 */     pub span: Span,
/* FP:hir.rs-2008 */     pub pat: &'hir Pat<'hir>,
/* FP:hir.rs-2009 */     pub ty: Option<&'hir Ty<'hir>>,
/* FP:hir.rs-2010 */     pub init: &'hir Expr<'hir>,
/* FP:hir.rs-2011 */     /// `Recovered::Yes` when this let expressions is not in a syntactically valid location.
/* FP:hir.rs-2012 */     /// Used to prevent building MIR in such situations.
/* FP:hir.rs-2013 */     pub recovered: ast::Recovered,
/* FP:hir.rs-2014 */ }
/* FP:hir.rs-2015 */ 
/* FP:hir.rs-2016 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-2017 */ pub struct ExprField<'hir> {
/* FP:hir.rs-2018 */     #[stable_hasher(ignore)]
/* FP:hir.rs-2019 */     pub hir_id: HirId,
/* FP:hir.rs-2020 */     pub ident: Ident,
/* FP:hir.rs-2021 */     pub expr: &'hir Expr<'hir>,
/* FP:hir.rs-2022 */     pub span: Span,
/* FP:hir.rs-2023 */     pub is_shorthand: bool,
/* FP:hir.rs-2024 */ }
/* FP:hir.rs-2025 */ 
/* FP:hir.rs-2026 */ #[derive(Copy, Clone, PartialEq, Debug, HashStable_Generic)]
/* FP:hir.rs-2027 */ pub enum BlockCheckMode {
/* FP:hir.rs-2028 */     DefaultBlock,
/* FP:hir.rs-2029 */     UnsafeBlock(UnsafeSource),
/* FP:hir.rs-2030 */ }
/* FP:hir.rs-2031 */ 
/* FP:hir.rs-2032 */ #[derive(Copy, Clone, PartialEq, Debug, HashStable_Generic)]
/* FP:hir.rs-2033 */ pub enum UnsafeSource {
/* FP:hir.rs-2034 */     CompilerGenerated,
/* FP:hir.rs-2035 */     UserProvided,
/* FP:hir.rs-2036 */ }
/* FP:hir.rs-2037 */ 
/* FP:hir.rs-2038 */ #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, HashStable_Generic)]
/* FP:hir.rs-2039 */ pub struct BodyId {
/* FP:hir.rs-2040 */     pub hir_id: HirId,
/* FP:hir.rs-2041 */ }
/* FP:hir.rs-2042 */ 
/* FP:hir.rs-2043 */ /// The body of a function, closure, or constant value. In the case of
/* FP:hir.rs-2044 */ /// a function, the body contains not only the function body itself
/* FP:hir.rs-2045 */ /// (which is an expression), but also the argument patterns, since
/* FP:hir.rs-2046 */ /// those are something that the caller doesn't really care about.
/* FP:hir.rs-2047 */ ///
/* FP:hir.rs-2048 */ /// # Examples
/* FP:hir.rs-2049 */ ///
/* FP:hir.rs-2050 */ /// ```
/* FP:hir.rs-2051 */ /// fn foo((x, y): (u32, u32)) -> u32 {
/* FP:hir.rs-2052 */ ///     x + y
/* FP:hir.rs-2053 */ /// }
/* FP:hir.rs-2054 */ /// ```
/* FP:hir.rs-2055 */ ///
/* FP:hir.rs-2056 */ /// Here, the `Body` associated with `foo()` would contain:
/* FP:hir.rs-2057 */ ///
/* FP:hir.rs-2058 */ /// - an `params` array containing the `(x, y)` pattern
/* FP:hir.rs-2059 */ /// - a `value` containing the `x + y` expression (maybe wrapped in a block)
/* FP:hir.rs-2060 */ /// - `coroutine_kind` would be `None`
/* FP:hir.rs-2061 */ ///
/* FP:hir.rs-2062 */ /// All bodies have an **owner**, which can be accessed via the HIR
/* FP:hir.rs-2063 */ /// map using `body_owner_def_id()`.
/* FP:hir.rs-2064 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-2065 */ pub struct Body<'hir> {
/* FP:hir.rs-2066 */     pub params: &'hir [Param<'hir>],
/* FP:hir.rs-2067 */     pub value: &'hir Expr<'hir>,
/* FP:hir.rs-2068 */ }
/* FP:hir.rs-2069 */ 
/* FP:hir.rs-2070 */ impl<'hir> Body<'hir> {
/* FP:hir.rs-2071 */     pub fn id(&self) -> BodyId {
/* FP:hir.rs-2072 */         BodyId { hir_id: self.value.hir_id }
/* FP:hir.rs-2073 */     }
/* FP:hir.rs-2074 */ }
/* FP:hir.rs-2075 */ 
/* FP:hir.rs-2076 */ /// The type of source expression that caused this coroutine to be created.
/* FP:hir.rs-2077 */ #[derive(Clone, PartialEq, Eq, Debug, Copy, Hash, HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-2078 */ pub enum CoroutineKind {
/* FP:hir.rs-2079 */     /// A coroutine that comes from a desugaring.
/* FP:hir.rs-2080 */     Desugared(CoroutineDesugaring, CoroutineSource),
/* FP:hir.rs-2081 */ 
/* FP:hir.rs-2082 */     /// A coroutine literal created via a `yield` inside a closure.
/* FP:hir.rs-2083 */     Coroutine(Movability),
/* FP:hir.rs-2084 */ }
/* FP:hir.rs-2085 */ 
/* FP:hir.rs-2086 */ impl CoroutineKind {
/* FP:hir.rs-2087 */     pub fn movability(self) -> Movability {
/* FP:hir.rs-2088 */         match self {
/* FP:hir.rs-2089 */             CoroutineKind::Desugared(CoroutineDesugaring::Async, _)
/* FP:hir.rs-2090 */             | CoroutineKind::Desugared(CoroutineDesugaring::AsyncGen, _) => Movability::Static,
/* FP:hir.rs-2091 */             CoroutineKind::Desugared(CoroutineDesugaring::Gen, _) => Movability::Movable,
/* FP:hir.rs-2092 */             CoroutineKind::Coroutine(mov) => mov,
/* FP:hir.rs-2093 */         }
/* FP:hir.rs-2094 */     }
/* FP:hir.rs-2095 */ 
/* FP:hir.rs-2096 */     pub fn is_fn_like(self) -> bool {
/* FP:hir.rs-2097 */         matches!(self, CoroutineKind::Desugared(_, CoroutineSource::Fn))
/* FP:hir.rs-2098 */     }
/* FP:hir.rs-2099 */ 
/* FP:hir.rs-2100 */     pub fn to_plural_string(&self) -> String {
/* FP:hir.rs-2101 */         match self {
/* FP:hir.rs-2102 */             CoroutineKind::Desugared(d, CoroutineSource::Fn) => format!("{d:#}fn bodies"),
/* FP:hir.rs-2103 */             CoroutineKind::Desugared(d, CoroutineSource::Block) => format!("{d:#}blocks"),
/* FP:hir.rs-2104 */             CoroutineKind::Desugared(d, CoroutineSource::Closure) => format!("{d:#}closure bodies"),
/* FP:hir.rs-2105 */             CoroutineKind::Coroutine(_) => "coroutines".to_string(),
/* FP:hir.rs-2106 */         }
/* FP:hir.rs-2107 */     }
/* FP:hir.rs-2108 */ }
/* FP:hir.rs-2109 */ 
/* FP:hir.rs-2110 */ impl fmt::Display for CoroutineKind {
/* FP:hir.rs-2111 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-2112 */         match self {
/* FP:hir.rs-2113 */             CoroutineKind::Desugared(d, k) => {
/* FP:hir.rs-2114 */                 d.fmt(f)?;
/* FP:hir.rs-2115 */                 k.fmt(f)
/* FP:hir.rs-2116 */             }
/* FP:hir.rs-2117 */             CoroutineKind::Coroutine(_) => f.write_str("coroutine"),
/* FP:hir.rs-2118 */         }
/* FP:hir.rs-2119 */     }
/* FP:hir.rs-2120 */ }
/* FP:hir.rs-2121 */ 
/* FP:hir.rs-2122 */ /// In the case of a coroutine created as part of an async/gen construct,
/* FP:hir.rs-2123 */ /// which kind of async/gen construct caused it to be created?
/* FP:hir.rs-2124 */ ///
/* FP:hir.rs-2125 */ /// This helps error messages but is also used to drive coercions in
/* FP:hir.rs-2126 */ /// type-checking (see #60424).
/* FP:hir.rs-2127 */ #[derive(Clone, PartialEq, Eq, Hash, Debug, Copy, HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-2128 */ pub enum CoroutineSource {
/* FP:hir.rs-2129 */     /// An explicit `async`/`gen` block written by the user.
/* FP:hir.rs-2130 */     Block,
/* FP:hir.rs-2131 */ 
/* FP:hir.rs-2132 */     /// An explicit `async`/`gen` closure written by the user.
/* FP:hir.rs-2133 */     Closure,
/* FP:hir.rs-2134 */ 
/* FP:hir.rs-2135 */     /// The `async`/`gen` block generated as the body of an async/gen function.
/* FP:hir.rs-2136 */     Fn,
/* FP:hir.rs-2137 */ }
/* FP:hir.rs-2138 */ 
/* FP:hir.rs-2139 */ impl fmt::Display for CoroutineSource {
/* FP:hir.rs-2140 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-2141 */         match self {
/* FP:hir.rs-2142 */             CoroutineSource::Block => "block",
/* FP:hir.rs-2143 */             CoroutineSource::Closure => "closure body",
/* FP:hir.rs-2144 */             CoroutineSource::Fn => "fn body",
/* FP:hir.rs-2145 */         }
/* FP:hir.rs-2146 */         .fmt(f)
/* FP:hir.rs-2147 */     }
/* FP:hir.rs-2148 */ }
/* FP:hir.rs-2149 */ 
/* FP:hir.rs-2150 */ #[derive(Clone, PartialEq, Eq, Debug, Copy, Hash, HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-2151 */ pub enum CoroutineDesugaring {
/* FP:hir.rs-2152 */     /// An explicit `async` block or the body of an `async` function.
/* FP:hir.rs-2153 */     Async,
/* FP:hir.rs-2154 */ 
/* FP:hir.rs-2155 */     /// An explicit `gen` block or the body of a `gen` function.
/* FP:hir.rs-2156 */     Gen,
/* FP:hir.rs-2157 */ 
/* FP:hir.rs-2158 */     /// An explicit `async gen` block or the body of an `async gen` function,
/* FP:hir.rs-2159 */     /// which is able to both `yield` and `.await`.
/* FP:hir.rs-2160 */     AsyncGen,
/* FP:hir.rs-2161 */ }
/* FP:hir.rs-2162 */ 
/* FP:hir.rs-2163 */ impl fmt::Display for CoroutineDesugaring {
/* FP:hir.rs-2164 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-2165 */         match self {
/* FP:hir.rs-2166 */             CoroutineDesugaring::Async => {
/* FP:hir.rs-2167 */                 if f.alternate() {
/* FP:hir.rs-2168 */                     f.write_str("`async` ")?;
/* FP:hir.rs-2169 */                 } else {
/* FP:hir.rs-2170 */                     f.write_str("async ")?
/* FP:hir.rs-2171 */                 }
/* FP:hir.rs-2172 */             }
/* FP:hir.rs-2173 */             CoroutineDesugaring::Gen => {
/* FP:hir.rs-2174 */                 if f.alternate() {
/* FP:hir.rs-2175 */                     f.write_str("`gen` ")?;
/* FP:hir.rs-2176 */                 } else {
/* FP:hir.rs-2177 */                     f.write_str("gen ")?
/* FP:hir.rs-2178 */                 }
/* FP:hir.rs-2179 */             }
/* FP:hir.rs-2180 */             CoroutineDesugaring::AsyncGen => {
/* FP:hir.rs-2181 */                 if f.alternate() {
/* FP:hir.rs-2182 */                     f.write_str("`async gen` ")?;
/* FP:hir.rs-2183 */                 } else {
/* FP:hir.rs-2184 */                     f.write_str("async gen ")?
/* FP:hir.rs-2185 */                 }
/* FP:hir.rs-2186 */             }
/* FP:hir.rs-2187 */         }
/* FP:hir.rs-2188 */ 
/* FP:hir.rs-2189 */         Ok(())
/* FP:hir.rs-2190 */     }
/* FP:hir.rs-2191 */ }
/* FP:hir.rs-2192 */ 
/* FP:hir.rs-2193 */ #[derive(Copy, Clone, Debug)]
/* FP:hir.rs-2194 */ pub enum BodyOwnerKind {
/* FP:hir.rs-2195 */     /// Functions and methods.
/* FP:hir.rs-2196 */     Fn,
/* FP:hir.rs-2197 */ 
/* FP:hir.rs-2198 */     /// Closures
/* FP:hir.rs-2199 */     Closure,
/* FP:hir.rs-2200 */ 
/* FP:hir.rs-2201 */     /// Constants and associated constants, also including inline constants.
/* FP:hir.rs-2202 */     Const { inline: bool },
/* FP:hir.rs-2203 */ 
/* FP:hir.rs-2204 */     /// Initializer of a `static` item.
/* FP:hir.rs-2205 */     Static(Mutability),
/* FP:hir.rs-2206 */ 
/* FP:hir.rs-2207 */     /// Fake body for a global asm to store its const-like value types.
/* FP:hir.rs-2208 */     GlobalAsm,
/* FP:hir.rs-2209 */ }
/* FP:hir.rs-2210 */ 
/* FP:hir.rs-2211 */ impl BodyOwnerKind {
/* FP:hir.rs-2212 */     pub fn is_fn_or_closure(self) -> bool {
/* FP:hir.rs-2213 */         match self {
/* FP:hir.rs-2214 */             BodyOwnerKind::Fn | BodyOwnerKind::Closure => true,
/* FP:hir.rs-2215 */             BodyOwnerKind::Const { .. } | BodyOwnerKind::Static(_) | BodyOwnerKind::GlobalAsm => {
/* FP:hir.rs-2216 */                 false
/* FP:hir.rs-2217 */             }
/* FP:hir.rs-2218 */         }
/* FP:hir.rs-2219 */     }
/* FP:hir.rs-2220 */ }
/* FP:hir.rs-2221 */ 
/* FP:hir.rs-2222 */ /// The kind of an item that requires const-checking.
/* FP:hir.rs-2223 */ #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/* FP:hir.rs-2224 */ pub enum ConstContext {
/* FP:hir.rs-2225 */     /// A `const fn`.
/* FP:hir.rs-2226 */     ConstFn,
/* FP:hir.rs-2227 */ 
/* FP:hir.rs-2228 */     /// A `static` or `static mut`.
/* FP:hir.rs-2229 */     Static(Mutability),
/* FP:hir.rs-2230 */ 
/* FP:hir.rs-2231 */     /// A `const`, associated `const`, or other const context.
/* FP:hir.rs-2232 */     ///
/* FP:hir.rs-2233 */     /// Other contexts include:
/* FP:hir.rs-2234 */     /// - Array length expressions
/* FP:hir.rs-2235 */     /// - Enum discriminants
/* FP:hir.rs-2236 */     /// - Const generics
/* FP:hir.rs-2237 */     ///
/* FP:hir.rs-2238 */     /// For the most part, other contexts are treated just like a regular `const`, so they are
/* FP:hir.rs-2239 */     /// lumped into the same category.
/* FP:hir.rs-2240 */     Const { inline: bool },
/* FP:hir.rs-2241 */ }
/* FP:hir.rs-2242 */ 
/* FP:hir.rs-2243 */ impl ConstContext {
/* FP:hir.rs-2244 */     /// A description of this const context that can appear between backticks in an error message.
/* FP:hir.rs-2245 */     ///
/* FP:hir.rs-2246 */     /// E.g. `const` or `static mut`.
/* FP:hir.rs-2247 */     pub fn keyword_name(self) -> &'static str {
/* FP:hir.rs-2248 */         match self {
/* FP:hir.rs-2249 */             Self::Const { .. } => "const",
/* FP:hir.rs-2250 */             Self::Static(Mutability::Not) => "static",
/* FP:hir.rs-2251 */             Self::Static(Mutability::Mut) => "static mut",
/* FP:hir.rs-2252 */             Self::ConstFn => "const fn",
/* FP:hir.rs-2253 */         }
/* FP:hir.rs-2254 */     }
/* FP:hir.rs-2255 */ }
/* FP:hir.rs-2256 */ 
/* FP:hir.rs-2257 */ /// A colloquial, trivially pluralizable description of this const context for use in error
/* FP:hir.rs-2258 */ /// messages.
/* FP:hir.rs-2259 */ impl fmt::Display for ConstContext {
/* FP:hir.rs-2260 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-2261 */         match *self {
/* FP:hir.rs-2262 */             Self::Const { .. } => write!(f, "constant"),
/* FP:hir.rs-2263 */             Self::Static(_) => write!(f, "static"),
/* FP:hir.rs-2264 */             Self::ConstFn => write!(f, "constant function"),
/* FP:hir.rs-2265 */         }
/* FP:hir.rs-2266 */     }
/* FP:hir.rs-2267 */ }
/* FP:hir.rs-2268 */ 
/* FP:hir.rs-2269 */ impl IntoDiagArg for ConstContext {
/* FP:hir.rs-2270 */     fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
/* FP:hir.rs-2271 */         DiagArgValue::Str(Cow::Borrowed(match self {
/* FP:hir.rs-2272 */             ConstContext::ConstFn => "const_fn",
/* FP:hir.rs-2273 */             ConstContext::Static(_) => "static",
/* FP:hir.rs-2274 */             ConstContext::Const { .. } => "const",
/* FP:hir.rs-2275 */         }))
/* FP:hir.rs-2276 */     }
/* FP:hir.rs-2277 */ }
/* FP:hir.rs-2278 */ 
/* FP:hir.rs-2279 */ /// A literal.
/* FP:hir.rs-2280 */ pub type Lit = Spanned<LitKind>;
/* FP:hir.rs-2281 */ 
/* FP:hir.rs-2282 */ /// A constant (expression) that's not an item or associated item,
/* FP:hir.rs-2283 */ /// but needs its own `DefId` for type-checking, const-eval, etc.
/* FP:hir.rs-2284 */ /// These are usually found nested inside types (e.g., array lengths)
/* FP:hir.rs-2285 */ /// or expressions (e.g., repeat counts), and also used to define
/* FP:hir.rs-2286 */ /// explicit discriminant values for enum variants.
/* FP:hir.rs-2287 */ ///
/* FP:hir.rs-2288 */ /// You can check if this anon const is a default in a const param
/* FP:hir.rs-2289 */ /// `const N: usize = { ... }` with `tcx.hir_opt_const_param_default_param_def_id(..)`
/* FP:hir.rs-2290 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-2291 */ pub struct AnonConst {
/* FP:hir.rs-2292 */     #[stable_hasher(ignore)]
/* FP:hir.rs-2293 */     pub hir_id: HirId,
/* FP:hir.rs-2294 */     pub def_id: LocalDefId,
/* FP:hir.rs-2295 */     pub body: BodyId,
/* FP:hir.rs-2296 */     pub span: Span,
/* FP:hir.rs-2297 */ }
/* FP:hir.rs-2298 */ 
/* FP:hir.rs-2299 */ /// An inline constant expression `const { something }`.
/* FP:hir.rs-2300 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-2301 */ pub struct ConstBlock {
/* FP:hir.rs-2302 */     #[stable_hasher(ignore)]
/* FP:hir.rs-2303 */     pub hir_id: HirId,
/* FP:hir.rs-2304 */     pub def_id: LocalDefId,
/* FP:hir.rs-2305 */     pub body: BodyId,
/* FP:hir.rs-2306 */ }
/* FP:hir.rs-2307 */ 
/* FP:hir.rs-2308 */ /// An expression.
/* FP:hir.rs-2309 */ ///
/* FP:hir.rs-2310 */ /// For more details, see the [rust lang reference].
/* FP:hir.rs-2311 */ /// Note that the reference does not document nightly-only features.
/* FP:hir.rs-2312 */ /// There may be also slight differences in the names and representation of AST nodes between
/* FP:hir.rs-2313 */ /// the compiler and the reference.
/* FP:hir.rs-2314 */ ///
/* FP:hir.rs-2315 */ /// [rust lang reference]: https://doc.rust-lang.org/reference/expressions.html
/* FP:hir.rs-2316 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-2317 */ pub struct Expr<'hir> {
/* FP:hir.rs-2318 */     #[stable_hasher(ignore)]
/* FP:hir.rs-2319 */     pub hir_id: HirId,
/* FP:hir.rs-2320 */     pub kind: ExprKind<'hir>,
/* FP:hir.rs-2321 */     pub span: Span,
/* FP:hir.rs-2322 */ }
/* FP:hir.rs-2323 */ 
/* FP:hir.rs-2324 */ impl Expr<'_> {
/* FP:hir.rs-2325 */     pub fn precedence(&self, has_attr: &dyn Fn(HirId) -> bool) -> ExprPrecedence {
/* FP:hir.rs-2326 */         let prefix_attrs_precedence = || -> ExprPrecedence {
/* FP:hir.rs-2327 */             if has_attr(self.hir_id) { ExprPrecedence::Prefix } else { ExprPrecedence::Unambiguous }
/* FP:hir.rs-2328 */         };
/* FP:hir.rs-2329 */ 
/* FP:hir.rs-2330 */         match &self.kind {
/* FP:hir.rs-2331 */             ExprKind::Closure(closure) => {
/* FP:hir.rs-2332 */                 match closure.fn_decl.output {
/* FP:hir.rs-2333 */                     FnRetTy::DefaultReturn(_) => ExprPrecedence::Jump,
/* FP:hir.rs-2334 */                     FnRetTy::Return(_) => prefix_attrs_precedence(),
/* FP:hir.rs-2335 */                 }
/* FP:hir.rs-2336 */             }
/* FP:hir.rs-2337 */ 
/* FP:hir.rs-2338 */             ExprKind::Break(..)
/* FP:hir.rs-2339 */             | ExprKind::Ret(..)
/* FP:hir.rs-2340 */             | ExprKind::Yield(..)
/* FP:hir.rs-2341 */             | ExprKind::Become(..) => ExprPrecedence::Jump,
/* FP:hir.rs-2342 */ 
/* FP:hir.rs-2343 */             // Binop-like expr kinds, handled by `AssocOp`.
/* FP:hir.rs-2344 */             ExprKind::Binary(op, ..) => op.node.precedence(),
/* FP:hir.rs-2345 */             ExprKind::Cast(..) => ExprPrecedence::Cast,
/* FP:hir.rs-2346 */ 
/* FP:hir.rs-2347 */             ExprKind::Assign(..) |
/* FP:hir.rs-2348 */             ExprKind::AssignOp(..) => ExprPrecedence::Assign,
/* FP:hir.rs-2349 */ 
/* FP:hir.rs-2350 */             // Unary, prefix
/* FP:hir.rs-2351 */             ExprKind::AddrOf(..)
/* FP:hir.rs-2352 */             // Here `let pats = expr` has `let pats =` as a "unary" prefix of `expr`.
/* FP:hir.rs-2353 */             // However, this is not exactly right. When `let _ = a` is the LHS of a binop we
/* FP:hir.rs-2354 */             // need parens sometimes. E.g. we can print `(let _ = a) && b` as `let _ = a && b`
/* FP:hir.rs-2355 */             // but we need to print `(let _ = a) < b` as-is with parens.
/* FP:hir.rs-2356 */             | ExprKind::Let(..)
/* FP:hir.rs-2357 */             | ExprKind::Unary(..) => ExprPrecedence::Prefix,
/* FP:hir.rs-2358 */ 
/* FP:hir.rs-2359 */             // Need parens if and only if there are prefix attributes.
/* FP:hir.rs-2360 */             ExprKind::Array(_)
/* FP:hir.rs-2361 */             | ExprKind::Block(..)
/* FP:hir.rs-2362 */             | ExprKind::Call(..)
/* FP:hir.rs-2363 */             | ExprKind::ConstBlock(_)
/* FP:hir.rs-2364 */             | ExprKind::Continue(..)
/* FP:hir.rs-2365 */             | ExprKind::Field(..)
/* FP:hir.rs-2366 */             | ExprKind::If(..)
/* FP:hir.rs-2367 */             | ExprKind::Index(..)
/* FP:hir.rs-2368 */             | ExprKind::InlineAsm(..)
/* FP:hir.rs-2369 */             | ExprKind::Lit(_)
/* FP:hir.rs-2370 */             | ExprKind::Loop(..)
/* FP:hir.rs-2371 */             | ExprKind::Match(..)
/* FP:hir.rs-2372 */             | ExprKind::MethodCall(..)
/* FP:hir.rs-2373 */             | ExprKind::OffsetOf(..)
/* FP:hir.rs-2374 */             | ExprKind::Path(..)
/* FP:hir.rs-2375 */             | ExprKind::Repeat(..)
/* FP:hir.rs-2376 */             | ExprKind::Struct(..)
/* FP:hir.rs-2377 */             | ExprKind::Tup(_)
/* FP:hir.rs-2378 */             | ExprKind::Type(..)
/* FP:hir.rs-2379 */             | ExprKind::UnsafeBinderCast(..)
/* FP:hir.rs-2380 */             | ExprKind::Use(..)
/* FP:hir.rs-2381 */             | ExprKind::Err(_) => prefix_attrs_precedence(),
/* FP:hir.rs-2382 */ 
/* FP:hir.rs-2383 */             ExprKind::DropTemps(expr, ..) => expr.precedence(has_attr),
/* FP:hir.rs-2384 */         }
/* FP:hir.rs-2385 */     }
/* FP:hir.rs-2386 */ 
/* FP:hir.rs-2387 */     /// Whether this looks like a place expr, without checking for deref
/* FP:hir.rs-2388 */     /// adjustments.
/* FP:hir.rs-2389 */     /// This will return `true` in some potentially surprising cases such as
/* FP:hir.rs-2390 */     /// `CONSTANT.field`.
/* FP:hir.rs-2391 */     pub fn is_syntactic_place_expr(&self) -> bool {
/* FP:hir.rs-2392 */         self.is_place_expr(|_| true)
/* FP:hir.rs-2393 */     }
/* FP:hir.rs-2394 */ 
/* FP:hir.rs-2395 */     /// Whether this is a place expression.
/* FP:hir.rs-2396 */     ///
/* FP:hir.rs-2397 */     /// `allow_projections_from` should return `true` if indexing a field or index expression based
/* FP:hir.rs-2398 */     /// on the given expression should be considered a place expression.
/* FP:hir.rs-2399 */     pub fn is_place_expr(&self, mut allow_projections_from: impl FnMut(&Self) -> bool) -> bool {
/* FP:hir.rs-2400 */         match self.kind {
/* FP:hir.rs-2401 */             ExprKind::Path(QPath::Resolved(_, ref path)) => {
/* FP:hir.rs-2402 */                 matches!(path.res, Res::Local(..) | Res::Def(DefKind::Static { .. }, _) | Res::Err)
/* FP:hir.rs-2403 */             }
/* FP:hir.rs-2404 */ 
/* FP:hir.rs-2405 */             // Type ascription inherits its place expression kind from its
/* FP:hir.rs-2406 */             // operand. See:
/* FP:hir.rs-2407 */             // https://github.com/rust-lang/rfcs/blob/master/text/0803-type-ascription.md#type-ascription-and-temporaries
/* FP:hir.rs-2408 */             ExprKind::Type(ref e, _) => e.is_place_expr(allow_projections_from),
/* FP:hir.rs-2409 */ 
/* FP:hir.rs-2410 */             // Unsafe binder cast preserves place-ness of the sub-expression.
/* FP:hir.rs-2411 */             ExprKind::UnsafeBinderCast(_, e, _) => e.is_place_expr(allow_projections_from),
/* FP:hir.rs-2412 */ 
/* FP:hir.rs-2413 */             ExprKind::Unary(UnOp::Deref, _) => true,
/* FP:hir.rs-2414 */ 
/* FP:hir.rs-2415 */             ExprKind::Field(ref base, _) | ExprKind::Index(ref base, _, _) => {
/* FP:hir.rs-2416 */                 allow_projections_from(base) || base.is_place_expr(allow_projections_from)
/* FP:hir.rs-2417 */             }
/* FP:hir.rs-2418 */ 
/* FP:hir.rs-2419 */             // Lang item paths cannot currently be local variables or statics.
/* FP:hir.rs-2420 */             ExprKind::Path(QPath::LangItem(..)) => false,
/* FP:hir.rs-2421 */ 
/* FP:hir.rs-2422 */             // Suppress errors for bad expressions.
/* FP:hir.rs-2423 */             ExprKind::Err(_guar)
/* FP:hir.rs-2424 */             | ExprKind::Let(&LetExpr { recovered: ast::Recovered::Yes(_guar), .. }) => true,
/* FP:hir.rs-2425 */ 
/* FP:hir.rs-2426 */             // Partially qualified paths in expressions can only legally
/* FP:hir.rs-2427 */             // refer to associated items which are always rvalues.
/* FP:hir.rs-2428 */             ExprKind::Path(QPath::TypeRelative(..))
/* FP:hir.rs-2429 */             | ExprKind::Call(..)
/* FP:hir.rs-2430 */             | ExprKind::MethodCall(..)
/* FP:hir.rs-2431 */             | ExprKind::Use(..)
/* FP:hir.rs-2432 */             | ExprKind::Struct(..)
/* FP:hir.rs-2433 */             | ExprKind::Tup(..)
/* FP:hir.rs-2434 */             | ExprKind::If(..)
/* FP:hir.rs-2435 */             | ExprKind::Match(..)
/* FP:hir.rs-2436 */             | ExprKind::Closure { .. }
/* FP:hir.rs-2437 */             | ExprKind::Block(..)
/* FP:hir.rs-2438 */             | ExprKind::Repeat(..)
/* FP:hir.rs-2439 */             | ExprKind::Array(..)
/* FP:hir.rs-2440 */             | ExprKind::Break(..)
/* FP:hir.rs-2441 */             | ExprKind::Continue(..)
/* FP:hir.rs-2442 */             | ExprKind::Ret(..)
/* FP:hir.rs-2443 */             | ExprKind::Become(..)
/* FP:hir.rs-2444 */             | ExprKind::Let(..)
/* FP:hir.rs-2445 */             | ExprKind::Loop(..)
/* FP:hir.rs-2446 */             | ExprKind::Assign(..)
/* FP:hir.rs-2447 */             | ExprKind::InlineAsm(..)
/* FP:hir.rs-2448 */             | ExprKind::OffsetOf(..)
/* FP:hir.rs-2449 */             | ExprKind::AssignOp(..)
/* FP:hir.rs-2450 */             | ExprKind::Lit(_)
/* FP:hir.rs-2451 */             | ExprKind::ConstBlock(..)
/* FP:hir.rs-2452 */             | ExprKind::Unary(..)
/* FP:hir.rs-2453 */             | ExprKind::AddrOf(..)
/* FP:hir.rs-2454 */             | ExprKind::Binary(..)
/* FP:hir.rs-2455 */             | ExprKind::Yield(..)
/* FP:hir.rs-2456 */             | ExprKind::Cast(..)
/* FP:hir.rs-2457 */             | ExprKind::DropTemps(..) => false,
/* FP:hir.rs-2458 */         }
/* FP:hir.rs-2459 */     }
/* FP:hir.rs-2460 */ 
/* FP:hir.rs-2461 */     /// Check if expression is an integer literal that can be used
/* FP:hir.rs-2462 */     /// where `usize` is expected.
/* FP:hir.rs-2463 */     pub fn is_size_lit(&self) -> bool {
/* FP:hir.rs-2464 */         matches!(
/* FP:hir.rs-2465 */             self.kind,
/* FP:hir.rs-2466 */             ExprKind::Lit(Lit {
/* FP:hir.rs-2467 */                 node: LitKind::Int(_, LitIntType::Unsuffixed | LitIntType::Unsigned(UintTy::Usize)),
/* FP:hir.rs-2468 */                 ..
/* FP:hir.rs-2469 */             })
/* FP:hir.rs-2470 */         )
/* FP:hir.rs-2471 */     }
/* FP:hir.rs-2472 */ 
/* FP:hir.rs-2473 */     /// If `Self.kind` is `ExprKind::DropTemps(expr)`, drill down until we get a non-`DropTemps`
/* FP:hir.rs-2474 */     /// `Expr`. This is used in suggestions to ignore this `ExprKind` as it is semantically
/* FP:hir.rs-2475 */     /// silent, only signaling the ownership system. By doing this, suggestions that check the
/* FP:hir.rs-2476 */     /// `ExprKind` of any given `Expr` for presentation don't have to care about `DropTemps`
/* FP:hir.rs-2477 */     /// beyond remembering to call this function before doing analysis on it.
/* FP:hir.rs-2478 */     pub fn peel_drop_temps(&self) -> &Self {
/* FP:hir.rs-2479 */         let mut expr = self;
/* FP:hir.rs-2480 */         while let ExprKind::DropTemps(inner) = &expr.kind {
/* FP:hir.rs-2481 */             expr = inner;
/* FP:hir.rs-2482 */         }
/* FP:hir.rs-2483 */         expr
/* FP:hir.rs-2484 */     }
/* FP:hir.rs-2485 */ 
/* FP:hir.rs-2486 */     pub fn peel_blocks(&self) -> &Self {
/* FP:hir.rs-2487 */         let mut expr = self;
/* FP:hir.rs-2488 */         while let ExprKind::Block(Block { expr: Some(inner), .. }, _) = &expr.kind {
/* FP:hir.rs-2489 */             expr = inner;
/* FP:hir.rs-2490 */         }
/* FP:hir.rs-2491 */         expr
/* FP:hir.rs-2492 */     }
/* FP:hir.rs-2493 */ 
/* FP:hir.rs-2494 */     pub fn peel_borrows(&self) -> &Self {
/* FP:hir.rs-2495 */         let mut expr = self;
/* FP:hir.rs-2496 */         while let ExprKind::AddrOf(.., inner) = &expr.kind {
/* FP:hir.rs-2497 */             expr = inner;
/* FP:hir.rs-2498 */         }
/* FP:hir.rs-2499 */         expr
/* FP:hir.rs-2500 */     }
/* FP:hir.rs-2501 */ 
/* FP:hir.rs-2502 */     pub fn can_have_side_effects(&self) -> bool {
/* FP:hir.rs-2503 */         match self.peel_drop_temps().kind {
/* FP:hir.rs-2504 */             ExprKind::Path(_) | ExprKind::Lit(_) | ExprKind::OffsetOf(..) | ExprKind::Use(..) => {
/* FP:hir.rs-2505 */                 false
/* FP:hir.rs-2506 */             }
/* FP:hir.rs-2507 */             ExprKind::Type(base, _)
/* FP:hir.rs-2508 */             | ExprKind::Unary(_, base)
/* FP:hir.rs-2509 */             | ExprKind::Field(base, _)
/* FP:hir.rs-2510 */             | ExprKind::Index(base, _, _)
/* FP:hir.rs-2511 */             | ExprKind::AddrOf(.., base)
/* FP:hir.rs-2512 */             | ExprKind::Cast(base, _)
/* FP:hir.rs-2513 */             | ExprKind::UnsafeBinderCast(_, base, _) => {
/* FP:hir.rs-2514 */                 // This isn't exactly true for `Index` and all `Unary`, but we are using this
/* FP:hir.rs-2515 */                 // method exclusively for diagnostics and there's a *cultural* pressure against
/* FP:hir.rs-2516 */                 // them being used only for its side-effects.
/* FP:hir.rs-2517 */                 base.can_have_side_effects()
/* FP:hir.rs-2518 */             }
/* FP:hir.rs-2519 */             ExprKind::Struct(_, fields, init) => {
/* FP:hir.rs-2520 */                 let init_side_effects = match init {
/* FP:hir.rs-2521 */                     StructTailExpr::Base(init) => init.can_have_side_effects(),
/* FP:hir.rs-2522 */                     StructTailExpr::DefaultFields(_) | StructTailExpr::None => false,
/* FP:hir.rs-2523 */                 };
/* FP:hir.rs-2524 */                 fields.iter().map(|field| field.expr).any(|e| e.can_have_side_effects())
/* FP:hir.rs-2525 */                     || init_side_effects
/* FP:hir.rs-2526 */             }
/* FP:hir.rs-2527 */ 
/* FP:hir.rs-2528 */             ExprKind::Array(args)
/* FP:hir.rs-2529 */             | ExprKind::Tup(args)
/* FP:hir.rs-2530 */             | ExprKind::Call(
/* FP:hir.rs-2531 */                 Expr {
/* FP:hir.rs-2532 */                     kind:
/* FP:hir.rs-2533 */                         ExprKind::Path(QPath::Resolved(
/* FP:hir.rs-2534 */                             None,
/* FP:hir.rs-2535 */                             Path { res: Res::Def(DefKind::Ctor(_, CtorKind::Fn), _), .. },
/* FP:hir.rs-2536 */                         )),
/* FP:hir.rs-2537 */                     ..
/* FP:hir.rs-2538 */                 },
/* FP:hir.rs-2539 */                 args,
/* FP:hir.rs-2540 */             ) => args.iter().any(|arg| arg.can_have_side_effects()),
/* FP:hir.rs-2541 */             ExprKind::If(..)
/* FP:hir.rs-2542 */             | ExprKind::Match(..)
/* FP:hir.rs-2543 */             | ExprKind::MethodCall(..)
/* FP:hir.rs-2544 */             | ExprKind::Call(..)
/* FP:hir.rs-2545 */             | ExprKind::Closure { .. }
/* FP:hir.rs-2546 */             | ExprKind::Block(..)
/* FP:hir.rs-2547 */             | ExprKind::Repeat(..)
/* FP:hir.rs-2548 */             | ExprKind::Break(..)
/* FP:hir.rs-2549 */             | ExprKind::Continue(..)
/* FP:hir.rs-2550 */             | ExprKind::Ret(..)
/* FP:hir.rs-2551 */             | ExprKind::Become(..)
/* FP:hir.rs-2552 */             | ExprKind::Let(..)
/* FP:hir.rs-2553 */             | ExprKind::Loop(..)
/* FP:hir.rs-2554 */             | ExprKind::Assign(..)
/* FP:hir.rs-2555 */             | ExprKind::InlineAsm(..)
/* FP:hir.rs-2556 */             | ExprKind::AssignOp(..)
/* FP:hir.rs-2557 */             | ExprKind::ConstBlock(..)
/* FP:hir.rs-2558 */             | ExprKind::Binary(..)
/* FP:hir.rs-2559 */             | ExprKind::Yield(..)
/* FP:hir.rs-2560 */             | ExprKind::DropTemps(..)
/* FP:hir.rs-2561 */             | ExprKind::Err(_) => true,
/* FP:hir.rs-2562 */         }
/* FP:hir.rs-2563 */     }
/* FP:hir.rs-2564 */ 
/* FP:hir.rs-2565 */     /// To a first-order approximation, is this a pattern?
/* FP:hir.rs-2566 */     pub fn is_approximately_pattern(&self) -> bool {
/* FP:hir.rs-2567 */         match &self.kind {
/* FP:hir.rs-2568 */             ExprKind::Array(_)
/* FP:hir.rs-2569 */             | ExprKind::Call(..)
/* FP:hir.rs-2570 */             | ExprKind::Tup(_)
/* FP:hir.rs-2571 */             | ExprKind::Lit(_)
/* FP:hir.rs-2572 */             | ExprKind::Path(_)
/* FP:hir.rs-2573 */             | ExprKind::Struct(..) => true,
/* FP:hir.rs-2574 */             _ => false,
/* FP:hir.rs-2575 */         }
/* FP:hir.rs-2576 */     }
/* FP:hir.rs-2577 */ 
/* FP:hir.rs-2578 */     /// Whether this and the `other` expression are the same for purposes of an indexing operation.
/* FP:hir.rs-2579 */     ///
/* FP:hir.rs-2580 */     /// This is only used for diagnostics to see if we have things like `foo[i]` where `foo` is
/* FP:hir.rs-2581 */     /// borrowed multiple times with `i`.
/* FP:hir.rs-2582 */     pub fn equivalent_for_indexing(&self, other: &Expr<'_>) -> bool {
/* FP:hir.rs-2583 */         match (self.kind, other.kind) {
/* FP:hir.rs-2584 */             (ExprKind::Lit(lit1), ExprKind::Lit(lit2)) => lit1.node == lit2.node,
/* FP:hir.rs-2585 */             (
/* FP:hir.rs-2586 */                 ExprKind::Path(QPath::LangItem(item1, _)),
/* FP:hir.rs-2587 */                 ExprKind::Path(QPath::LangItem(item2, _)),
/* FP:hir.rs-2588 */             ) => item1 == item2,
/* FP:hir.rs-2589 */             (
/* FP:hir.rs-2590 */                 ExprKind::Path(QPath::Resolved(None, path1)),
/* FP:hir.rs-2591 */                 ExprKind::Path(QPath::Resolved(None, path2)),
/* FP:hir.rs-2592 */             ) => path1.res == path2.res,
/* FP:hir.rs-2593 */             (
/* FP:hir.rs-2594 */                 ExprKind::Struct(
/* FP:hir.rs-2595 */                     QPath::LangItem(LangItem::RangeTo, _),
/* FP:hir.rs-2596 */                     [val1],
/* FP:hir.rs-2597 */                     StructTailExpr::None,
/* FP:hir.rs-2598 */                 ),
/* FP:hir.rs-2599 */                 ExprKind::Struct(
/* FP:hir.rs-2600 */                     QPath::LangItem(LangItem::RangeTo, _),
/* FP:hir.rs-2601 */                     [val2],
/* FP:hir.rs-2602 */                     StructTailExpr::None,
/* FP:hir.rs-2603 */                 ),
/* FP:hir.rs-2604 */             )
/* FP:hir.rs-2605 */             | (
/* FP:hir.rs-2606 */                 ExprKind::Struct(
/* FP:hir.rs-2607 */                     QPath::LangItem(LangItem::RangeToInclusive, _),
/* FP:hir.rs-2608 */                     [val1],
/* FP:hir.rs-2609 */                     StructTailExpr::None,
/* FP:hir.rs-2610 */                 ),
/* FP:hir.rs-2611 */                 ExprKind::Struct(
/* FP:hir.rs-2612 */                     QPath::LangItem(LangItem::RangeToInclusive, _),
/* FP:hir.rs-2613 */                     [val2],
/* FP:hir.rs-2614 */                     StructTailExpr::None,
/* FP:hir.rs-2615 */                 ),
/* FP:hir.rs-2616 */             )
/* FP:hir.rs-2617 */             | (
/* FP:hir.rs-2618 */                 ExprKind::Struct(
/* FP:hir.rs-2619 */                     QPath::LangItem(LangItem::RangeToInclusiveCopy, _),
/* FP:hir.rs-2620 */                     [val1],
/* FP:hir.rs-2621 */                     StructTailExpr::None,
/* FP:hir.rs-2622 */                 ),
/* FP:hir.rs-2623 */                 ExprKind::Struct(
/* FP:hir.rs-2624 */                     QPath::LangItem(LangItem::RangeToInclusiveCopy, _),
/* FP:hir.rs-2625 */                     [val2],
/* FP:hir.rs-2626 */                     StructTailExpr::None,
/* FP:hir.rs-2627 */                 ),
/* FP:hir.rs-2628 */             )
/* FP:hir.rs-2629 */             | (
/* FP:hir.rs-2630 */                 ExprKind::Struct(
/* FP:hir.rs-2631 */                     QPath::LangItem(LangItem::RangeFrom, _),
/* FP:hir.rs-2632 */                     [val1],
/* FP:hir.rs-2633 */                     StructTailExpr::None,
/* FP:hir.rs-2634 */                 ),
/* FP:hir.rs-2635 */                 ExprKind::Struct(
/* FP:hir.rs-2636 */                     QPath::LangItem(LangItem::RangeFrom, _),
/* FP:hir.rs-2637 */                     [val2],
/* FP:hir.rs-2638 */                     StructTailExpr::None,
/* FP:hir.rs-2639 */                 ),
/* FP:hir.rs-2640 */             )
/* FP:hir.rs-2641 */             | (
/* FP:hir.rs-2642 */                 ExprKind::Struct(
/* FP:hir.rs-2643 */                     QPath::LangItem(LangItem::RangeFromCopy, _),
/* FP:hir.rs-2644 */                     [val1],
/* FP:hir.rs-2645 */                     StructTailExpr::None,
/* FP:hir.rs-2646 */                 ),
/* FP:hir.rs-2647 */                 ExprKind::Struct(
/* FP:hir.rs-2648 */                     QPath::LangItem(LangItem::RangeFromCopy, _),
/* FP:hir.rs-2649 */                     [val2],
/* FP:hir.rs-2650 */                     StructTailExpr::None,
/* FP:hir.rs-2651 */                 ),
/* FP:hir.rs-2652 */             ) => val1.expr.equivalent_for_indexing(val2.expr),
/* FP:hir.rs-2653 */             (
/* FP:hir.rs-2654 */                 ExprKind::Struct(
/* FP:hir.rs-2655 */                     QPath::LangItem(LangItem::Range, _),
/* FP:hir.rs-2656 */                     [val1, val3],
/* FP:hir.rs-2657 */                     StructTailExpr::None,
/* FP:hir.rs-2658 */                 ),
/* FP:hir.rs-2659 */                 ExprKind::Struct(
/* FP:hir.rs-2660 */                     QPath::LangItem(LangItem::Range, _),
/* FP:hir.rs-2661 */                     [val2, val4],
/* FP:hir.rs-2662 */                     StructTailExpr::None,
/* FP:hir.rs-2663 */                 ),
/* FP:hir.rs-2664 */             )
/* FP:hir.rs-2665 */             | (
/* FP:hir.rs-2666 */                 ExprKind::Struct(
/* FP:hir.rs-2667 */                     QPath::LangItem(LangItem::RangeCopy, _),
/* FP:hir.rs-2668 */                     [val1, val3],
/* FP:hir.rs-2669 */                     StructTailExpr::None,
/* FP:hir.rs-2670 */                 ),
/* FP:hir.rs-2671 */                 ExprKind::Struct(
/* FP:hir.rs-2672 */                     QPath::LangItem(LangItem::RangeCopy, _),
/* FP:hir.rs-2673 */                     [val2, val4],
/* FP:hir.rs-2674 */                     StructTailExpr::None,
/* FP:hir.rs-2675 */                 ),
/* FP:hir.rs-2676 */             )
/* FP:hir.rs-2677 */             | (
/* FP:hir.rs-2678 */                 ExprKind::Struct(
/* FP:hir.rs-2679 */                     QPath::LangItem(LangItem::RangeInclusiveCopy, _),
/* FP:hir.rs-2680 */                     [val1, val3],
/* FP:hir.rs-2681 */                     StructTailExpr::None,
/* FP:hir.rs-2682 */                 ),
/* FP:hir.rs-2683 */                 ExprKind::Struct(
/* FP:hir.rs-2684 */                     QPath::LangItem(LangItem::RangeInclusiveCopy, _),
/* FP:hir.rs-2685 */                     [val2, val4],
/* FP:hir.rs-2686 */                     StructTailExpr::None,
/* FP:hir.rs-2687 */                 ),
/* FP:hir.rs-2688 */             ) => {
/* FP:hir.rs-2689 */                 val1.expr.equivalent_for_indexing(val2.expr)
/* FP:hir.rs-2690 */                     && val3.expr.equivalent_for_indexing(val4.expr)
/* FP:hir.rs-2691 */             }
/* FP:hir.rs-2692 */             _ => false,
/* FP:hir.rs-2693 */         }
/* FP:hir.rs-2694 */     }
/* FP:hir.rs-2695 */ 
/* FP:hir.rs-2696 */     pub fn method_ident(&self) -> Option<Ident> {
/* FP:hir.rs-2697 */         match self.kind {
/* FP:hir.rs-2698 */             ExprKind::MethodCall(receiver_method, ..) => Some(receiver_method.ident),
/* FP:hir.rs-2699 */             ExprKind::Unary(_, expr) | ExprKind::AddrOf(.., expr) => expr.method_ident(),
/* FP:hir.rs-2700 */             _ => None,
/* FP:hir.rs-2701 */         }
/* FP:hir.rs-2702 */     }
/* FP:hir.rs-2703 */ }
/* FP:hir.rs-2704 */ 
/* FP:hir.rs-2705 */ /// Checks if the specified expression is a built-in range literal.
/* FP:hir.rs-2706 */ /// (See: `LoweringContext::lower_expr()`).
/* FP:hir.rs-2707 */ pub fn is_range_literal(expr: &Expr<'_>) -> bool {
/* FP:hir.rs-2708 */     match expr.kind {
/* FP:hir.rs-2709 */         // All built-in range literals but `..=` and `..` desugar to `Struct`s.
/* FP:hir.rs-2710 */         ExprKind::Struct(ref qpath, _, _) => matches!(
/* FP:hir.rs-2711 */             **qpath,
/* FP:hir.rs-2712 */             QPath::LangItem(
/* FP:hir.rs-2713 */                 LangItem::Range
/* FP:hir.rs-2714 */                     | LangItem::RangeTo
/* FP:hir.rs-2715 */                     | LangItem::RangeFrom
/* FP:hir.rs-2716 */                     | LangItem::RangeFull
/* FP:hir.rs-2717 */                     | LangItem::RangeToInclusive
/* FP:hir.rs-2718 */                     | LangItem::RangeCopy
/* FP:hir.rs-2719 */                     | LangItem::RangeFromCopy
/* FP:hir.rs-2720 */                     | LangItem::RangeInclusiveCopy
/* FP:hir.rs-2721 */                     | LangItem::RangeToInclusiveCopy,
/* FP:hir.rs-2722 */                 ..
/* FP:hir.rs-2723 */             )
/* FP:hir.rs-2724 */         ),
/* FP:hir.rs-2725 */ 
/* FP:hir.rs-2726 */         // `..=` desugars into `::std::ops::RangeInclusive::new(...)`.
/* FP:hir.rs-2727 */         ExprKind::Call(ref func, _) => {
/* FP:hir.rs-2728 */             matches!(func.kind, ExprKind::Path(QPath::LangItem(LangItem::RangeInclusiveNew, ..)))
/* FP:hir.rs-2729 */         }
/* FP:hir.rs-2730 */ 
/* FP:hir.rs-2731 */         _ => false,
/* FP:hir.rs-2732 */     }
/* FP:hir.rs-2733 */ }
/* FP:hir.rs-2734 */ 
/* FP:hir.rs-2735 */ /// Checks if the specified expression needs parentheses for prefix
/* FP:hir.rs-2736 */ /// or postfix suggestions to be valid.
/* FP:hir.rs-2737 */ /// For example, `a + b` requires parentheses to suggest `&(a + b)`,
/* FP:hir.rs-2738 */ /// but just `a` does not.
/* FP:hir.rs-2739 */ /// Similarly, `(a + b).c()` also requires parentheses.
/* FP:hir.rs-2740 */ /// This should not be used for other types of suggestions.
/* FP:hir.rs-2741 */ pub fn expr_needs_parens(expr: &Expr<'_>) -> bool {
/* FP:hir.rs-2742 */     match expr.kind {
/* FP:hir.rs-2743 */         // parenthesize if needed (Issue #46756)
/* FP:hir.rs-2744 */         ExprKind::Cast(_, _) | ExprKind::Binary(_, _, _) => true,
/* FP:hir.rs-2745 */         // parenthesize borrows of range literals (Issue #54505)
/* FP:hir.rs-2746 */         _ if is_range_literal(expr) => true,
/* FP:hir.rs-2747 */         _ => false,
/* FP:hir.rs-2748 */     }
/* FP:hir.rs-2749 */ }
/* FP:hir.rs-2750 */ 
/* FP:hir.rs-2751 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-2752 */ pub enum ExprKind<'hir> {
/* FP:hir.rs-2753 */     /// Allow anonymous constants from an inline `const` block
/* FP:hir.rs-2754 */     ConstBlock(ConstBlock),
/* FP:hir.rs-2755 */     /// An array (e.g., `[a, b, c, d]`).
/* FP:hir.rs-2756 */     Array(&'hir [Expr<'hir>]),
/* FP:hir.rs-2757 */     /// A function call.
/* FP:hir.rs-2758 */     ///
/* FP:hir.rs-2759 */     /// The first field resolves to the function itself (usually an `ExprKind::Path`),
/* FP:hir.rs-2760 */     /// and the second field is the list of arguments.
/* FP:hir.rs-2761 */     /// This also represents calling the constructor of
/* FP:hir.rs-2762 */     /// tuple-like ADTs such as tuple structs and enum variants.
/* FP:hir.rs-2763 */     Call(&'hir Expr<'hir>, &'hir [Expr<'hir>]),
/* FP:hir.rs-2764 */     /// A method call (e.g., `x.foo::<'static, Bar, Baz>(a, b, c, d)`).
/* FP:hir.rs-2765 */     ///
/* FP:hir.rs-2766 */     /// The `PathSegment` represents the method name and its generic arguments
/* FP:hir.rs-2767 */     /// (within the angle brackets).
/* FP:hir.rs-2768 */     /// The `&Expr` is the expression that evaluates
/* FP:hir.rs-2769 */     /// to the object on which the method is being called on (the receiver),
/* FP:hir.rs-2770 */     /// and the `&[Expr]` is the rest of the arguments.
/* FP:hir.rs-2771 */     /// Thus, `x.foo::<Bar, Baz>(a, b, c, d)` is represented as
/* FP:hir.rs-2772 */     /// `ExprKind::MethodCall(PathSegment { foo, [Bar, Baz] }, x, [a, b, c, d], span)`.
/* FP:hir.rs-2773 */     /// The final `Span` represents the span of the function and arguments
/* FP:hir.rs-2774 */     /// (e.g. `foo::<Bar, Baz>(a, b, c, d)` in `x.foo::<Bar, Baz>(a, b, c, d)`
/* FP:hir.rs-2775 */     ///
/* FP:hir.rs-2776 */     /// To resolve the called method to a `DefId`, call [`type_dependent_def_id`] with
/* FP:hir.rs-2777 */     /// the `hir_id` of the `MethodCall` node itself.
/* FP:hir.rs-2778 */     ///
/* FP:hir.rs-2779 */     /// [`type_dependent_def_id`]: ../../rustc_middle/ty/struct.TypeckResults.html#method.type_dependent_def_id
/* FP:hir.rs-2780 */     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
/* FP:hir.rs-2781 */     /// An use expression (e.g., `var.use`).
/* FP:hir.rs-2782 */     Use(&'hir Expr<'hir>, Span),
/* FP:hir.rs-2783 */     /// A tuple (e.g., `(a, b, c, d)`).
/* FP:hir.rs-2784 */     Tup(&'hir [Expr<'hir>]),
/* FP:hir.rs-2785 */     /// A binary operation (e.g., `a + b`, `a * b`).
/* FP:hir.rs-2786 */     Binary(BinOp, &'hir Expr<'hir>, &'hir Expr<'hir>),
/* FP:hir.rs-2787 */     /// A unary operation (e.g., `!x`, `*x`).
/* FP:hir.rs-2788 */     Unary(UnOp, &'hir Expr<'hir>),
/* FP:hir.rs-2789 */     /// A literal (e.g., `1`, `"foo"`).
/* FP:hir.rs-2790 */     Lit(Lit),
/* FP:hir.rs-2791 */     /// A cast (e.g., `foo as f64`).
/* FP:hir.rs-2792 */     Cast(&'hir Expr<'hir>, &'hir Ty<'hir>),
/* FP:hir.rs-2793 */     /// A type ascription (e.g., `x: Foo`). See RFC 3307.
/* FP:hir.rs-2794 */     Type(&'hir Expr<'hir>, &'hir Ty<'hir>),
/* FP:hir.rs-2795 */     /// Wraps the expression in a terminating scope.
/* FP:hir.rs-2796 */     /// This makes it semantically equivalent to `{ let _t = expr; _t }`.
/* FP:hir.rs-2797 */     ///
/* FP:hir.rs-2798 */     /// This construct only exists to tweak the drop order in AST lowering.
/* FP:hir.rs-2799 */     /// An example of that is the desugaring of `for` loops.
/* FP:hir.rs-2800 */     DropTemps(&'hir Expr<'hir>),
/* FP:hir.rs-2801 */     /// A `let $pat = $expr` expression.
/* FP:hir.rs-2802 */     ///
/* FP:hir.rs-2803 */     /// These are not [`LetStmt`] and only occur as expressions.
/* FP:hir.rs-2804 */     /// The `let Some(x) = foo()` in `if let Some(x) = foo()` is an example of `Let(..)`.
/* FP:hir.rs-2805 */     Let(&'hir LetExpr<'hir>),
/* FP:hir.rs-2806 */     /// An `if` block, with an optional else block.
/* FP:hir.rs-2807 */     ///
/* FP:hir.rs-2808 */     /// I.e., `if <expr> { <expr> } else { <expr> }`.
/* FP:hir.rs-2809 */     ///
/* FP:hir.rs-2810 */     /// The "then" expr is always `ExprKind::Block`. If present, the "else" expr is always
/* FP:hir.rs-2811 */     /// `ExprKind::Block` (for `else`) or `ExprKind::If` (for `else if`).
/* FP:hir.rs-2812 */     /// Note that using an `Expr` instead of a `Block` for the "then" part is intentional,
/* FP:hir.rs-2813 */     /// as it simplifies the type coercion machinery.
/* FP:hir.rs-2814 */     If(&'hir Expr<'hir>, &'hir Expr<'hir>, Option<&'hir Expr<'hir>>),
/* FP:hir.rs-2815 */     /// A conditionless loop (can be exited with `break`, `continue`, or `return`).
/* FP:hir.rs-2816 */     ///
/* FP:hir.rs-2817 */     /// I.e., `'label: loop { <block> }`.
/* FP:hir.rs-2818 */     ///
/* FP:hir.rs-2819 */     /// The `Span` is the loop header (`for x in y`/`while let pat = expr`).
/* FP:hir.rs-2820 */     Loop(&'hir Block<'hir>, Option<Label>, LoopSource, Span),
/* FP:hir.rs-2821 */     /// A `match` block, with a source that indicates whether or not it is
/* FP:hir.rs-2822 */     /// the result of a desugaring, and if so, which kind.
/* FP:hir.rs-2823 */     Match(&'hir Expr<'hir>, &'hir [Arm<'hir>], MatchSource),
/* FP:hir.rs-2824 */     /// A closure (e.g., `move |a, b, c| {a + b + c}`).
/* FP:hir.rs-2825 */     ///
/* FP:hir.rs-2826 */     /// The `Span` is the argument block `|...|`.
/* FP:hir.rs-2827 */     ///
/* FP:hir.rs-2828 */     /// This may also be a coroutine literal or an `async block` as indicated by the
/* FP:hir.rs-2829 */     /// `Option<Movability>`.
/* FP:hir.rs-2830 */     Closure(&'hir Closure<'hir>),
/* FP:hir.rs-2831 */     /// A block (e.g., `'label: { ... }`).
/* FP:hir.rs-2832 */     Block(&'hir Block<'hir>, Option<Label>),
/* FP:hir.rs-2833 */ 
/* FP:hir.rs-2834 */     /// An assignment (e.g., `a = foo()`).
/* FP:hir.rs-2835 */     Assign(&'hir Expr<'hir>, &'hir Expr<'hir>, Span),
/* FP:hir.rs-2836 */     /// An assignment with an operator.
/* FP:hir.rs-2837 */     ///
/* FP:hir.rs-2838 */     /// E.g., `a += 1`.
/* FP:hir.rs-2839 */     AssignOp(AssignOp, &'hir Expr<'hir>, &'hir Expr<'hir>),
/* FP:hir.rs-2840 */     /// Access of a named (e.g., `obj.foo`) or unnamed (e.g., `obj.0`) struct or tuple field.
/* FP:hir.rs-2841 */     Field(&'hir Expr<'hir>, Ident),
/* FP:hir.rs-2842 */     /// An indexing operation (`foo[2]`).
/* FP:hir.rs-2843 */     /// Similar to [`ExprKind::MethodCall`], the final `Span` represents the span of the brackets
/* FP:hir.rs-2844 */     /// and index.
/* FP:hir.rs-2845 */     Index(&'hir Expr<'hir>, &'hir Expr<'hir>, Span),
/* FP:hir.rs-2846 */ 
/* FP:hir.rs-2847 */     /// Path to a definition, possibly containing lifetime or type parameters.
/* FP:hir.rs-2848 */     Path(QPath<'hir>),
/* FP:hir.rs-2849 */ 
/* FP:hir.rs-2850 */     /// A referencing operation (i.e., `&a` or `&mut a`).
/* FP:hir.rs-2851 */     AddrOf(BorrowKind, Mutability, &'hir Expr<'hir>),
/* FP:hir.rs-2852 */     /// A `break`, with an optional label to break.
/* FP:hir.rs-2853 */     Break(Destination, Option<&'hir Expr<'hir>>),
/* FP:hir.rs-2854 */     /// A `continue`, with an optional label.
/* FP:hir.rs-2855 */     Continue(Destination),
/* FP:hir.rs-2856 */     /// A `return`, with an optional value to be returned.
/* FP:hir.rs-2857 */     Ret(Option<&'hir Expr<'hir>>),
/* FP:hir.rs-2858 */     /// A `become`, with the value to be returned.
/* FP:hir.rs-2859 */     Become(&'hir Expr<'hir>),
/* FP:hir.rs-2860 */ 
/* FP:hir.rs-2861 */     /// Inline assembly (from `asm!`), with its outputs and inputs.
/* FP:hir.rs-2862 */     InlineAsm(&'hir InlineAsm<'hir>),
/* FP:hir.rs-2863 */ 
/* FP:hir.rs-2864 */     /// Field offset (`offset_of!`)
/* FP:hir.rs-2865 */     OffsetOf(&'hir Ty<'hir>, &'hir [Ident]),
/* FP:hir.rs-2866 */ 
/* FP:hir.rs-2867 */     /// A struct or struct-like variant literal expression.
/* FP:hir.rs-2868 */     ///
/* FP:hir.rs-2869 */     /// E.g., `Foo {x: 1, y: 2}`, or `Foo {x: 1, .. base}`,
/* FP:hir.rs-2870 */     /// where `base` is the `Option<Expr>`.
/* FP:hir.rs-2871 */     Struct(&'hir QPath<'hir>, &'hir [ExprField<'hir>], StructTailExpr<'hir>),
/* FP:hir.rs-2872 */ 
/* FP:hir.rs-2873 */     /// An array literal constructed from one repeated element.
/* FP:hir.rs-2874 */     ///
/* FP:hir.rs-2875 */     /// E.g., `[1; 5]`. The first expression is the element
/* FP:hir.rs-2876 */     /// to be repeated; the second is the number of times to repeat it.
/* FP:hir.rs-2877 */     Repeat(&'hir Expr<'hir>, &'hir ConstArg<'hir>),
/* FP:hir.rs-2878 */ 
/* FP:hir.rs-2879 */     /// A suspension point for coroutines (i.e., `yield <expr>`).
/* FP:hir.rs-2880 */     Yield(&'hir Expr<'hir>, YieldSource),
/* FP:hir.rs-2881 */ 
/* FP:hir.rs-2882 */     /// Operators which can be used to interconvert `unsafe` binder types.
/* FP:hir.rs-2883 */     /// e.g. `unsafe<'a> &'a i32` <=> `&i32`.
/* FP:hir.rs-2884 */     UnsafeBinderCast(UnsafeBinderCastKind, &'hir Expr<'hir>, Option<&'hir Ty<'hir>>),
/* FP:hir.rs-2885 */ 
/* FP:hir.rs-2886 */     /// A placeholder for an expression that wasn't syntactically well formed in some way.
/* FP:hir.rs-2887 */     Err(crate::rustc_span::ErrorGuaranteed),
/* FP:hir.rs-2888 */ }
/* FP:hir.rs-2889 */ 
/* FP:hir.rs-2890 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-2891 */ pub enum StructTailExpr<'hir> {
/* FP:hir.rs-2892 */     /// A struct expression where all the fields are explicitly enumerated: `Foo { a, b }`.
/* FP:hir.rs-2893 */     None,
/* FP:hir.rs-2894 */     /// A struct expression with a "base", an expression of the same type as the outer struct that
/* FP:hir.rs-2895 */     /// will be used to populate any fields not explicitly mentioned: `Foo { ..base }`
/* FP:hir.rs-2896 */     Base(&'hir Expr<'hir>),
/* FP:hir.rs-2897 */     /// A struct expression with a `..` tail but no "base" expression. The values from the struct
/* FP:hir.rs-2898 */     /// fields' default values will be used to populate any fields not explicitly mentioned:
/* FP:hir.rs-2899 */     /// `Foo { .. }`.
/* FP:hir.rs-2900 */     DefaultFields(Span),
/* FP:hir.rs-2901 */ }
/* FP:hir.rs-2902 */ 
/* FP:hir.rs-2903 */ /// Represents an optionally `Self`-qualified value/type path or associated extension.
/* FP:hir.rs-2904 */ ///
/* FP:hir.rs-2905 */ /// To resolve the path to a `DefId`, call [`qpath_res`].
/* FP:hir.rs-2906 */ ///
/* FP:hir.rs-2907 */ /// [`qpath_res`]: ../../rustc_middle/ty/struct.TypeckResults.html#method.qpath_res
/* FP:hir.rs-2908 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-2909 */ pub enum QPath<'hir> {
/* FP:hir.rs-2910 */     /// Path to a definition, optionally "fully-qualified" with a `Self`
/* FP:hir.rs-2911 */     /// type, if the path points to an associated item in a trait.
/* FP:hir.rs-2912 */     ///
/* FP:hir.rs-2913 */     /// E.g., an unqualified path like `Clone::clone` has `None` for `Self`,
/* FP:hir.rs-2914 */     /// while `<Vec<T> as Clone>::clone` has `Some(Vec<T>)` for `Self`,
/* FP:hir.rs-2915 */     /// even though they both have the same two-segment `Clone::clone` `Path`.
/* FP:hir.rs-2916 */     Resolved(Option<&'hir Ty<'hir>>, &'hir Path<'hir>),
/* FP:hir.rs-2917 */ 
/* FP:hir.rs-2918 */     /// Type-related paths (e.g., `<T>::default` or `<T>::Output`).
/* FP:hir.rs-2919 */     /// Will be resolved by type-checking to an associated item.
/* FP:hir.rs-2920 */     ///
/* FP:hir.rs-2921 */     /// UFCS source paths can desugar into this, with `Vec::new` turning into
/* FP:hir.rs-2922 */     /// `<Vec>::new`, and `T::X::Y::method` into `<<<T>::X>::Y>::method`,
/* FP:hir.rs-2923 */     /// the `X` and `Y` nodes each being a `TyKind::Path(QPath::TypeRelative(..))`.
/* FP:hir.rs-2924 */     TypeRelative(&'hir Ty<'hir>, &'hir PathSegment<'hir>),
/* FP:hir.rs-2925 */ 
/* FP:hir.rs-2926 */     /// Reference to a `#[lang = "foo"]` item.
/* FP:hir.rs-2927 */     LangItem(LangItem, Span),
/* FP:hir.rs-2928 */ }
/* FP:hir.rs-2929 */ 
/* FP:hir.rs-2930 */ impl<'hir> QPath<'hir> {
/* FP:hir.rs-2931 */     /// Returns the span of this `QPath`.
/* FP:hir.rs-2932 */     pub fn span(&self) -> Span {
/* FP:hir.rs-2933 */         match *self {
/* FP:hir.rs-2934 */             QPath::Resolved(_, path) => path.span,
/* FP:hir.rs-2935 */             QPath::TypeRelative(qself, ps) => qself.span.to(ps.ident.span),
/* FP:hir.rs-2936 */             QPath::LangItem(_, span) => span,
/* FP:hir.rs-2937 */         }
/* FP:hir.rs-2938 */     }
/* FP:hir.rs-2939 */ 
/* FP:hir.rs-2940 */     /// Returns the span of the qself of this `QPath`. For example, `()` in
/* FP:hir.rs-2941 */     /// `<() as Trait>::method`.
/* FP:hir.rs-2942 */     pub fn qself_span(&self) -> Span {
/* FP:hir.rs-2943 */         match *self {
/* FP:hir.rs-2944 */             QPath::Resolved(_, path) => path.span,
/* FP:hir.rs-2945 */             QPath::TypeRelative(qself, _) => qself.span,
/* FP:hir.rs-2946 */             QPath::LangItem(_, span) => span,
/* FP:hir.rs-2947 */         }
/* FP:hir.rs-2948 */     }
/* FP:hir.rs-2949 */ }
/* FP:hir.rs-2950 */ 
/* FP:hir.rs-2951 */ /// Hints at the original code for a let statement.
/* FP:hir.rs-2952 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-2953 */ pub enum LocalSource {
/* FP:hir.rs-2954 */     /// A `match _ { .. }`.
/* FP:hir.rs-2955 */     Normal,
/* FP:hir.rs-2956 */     /// When lowering async functions, we create locals within the `async move` so that
/* FP:hir.rs-2957 */     /// all parameters are dropped after the future is polled.
/* FP:hir.rs-2958 */     ///
/* FP:hir.rs-2959 */     /// ```ignore (pseudo-Rust)
/* FP:hir.rs-2960 */     /// async fn foo(<pattern> @ x: Type) {
/* FP:hir.rs-2961 */     ///     async move {
/* FP:hir.rs-2962 */     ///         let <pattern> = x;
/* FP:hir.rs-2963 */     ///     }
/* FP:hir.rs-2964 */     /// }
/* FP:hir.rs-2965 */     /// ```
/* FP:hir.rs-2966 */     AsyncFn,
/* FP:hir.rs-2967 */     /// A desugared `<expr>.await`.
/* FP:hir.rs-2968 */     AwaitDesugar,
/* FP:hir.rs-2969 */     /// A desugared `expr = expr`, where the LHS is a tuple, struct, array or underscore expression.
/* FP:hir.rs-2970 */     /// The span is that of the `=` sign.
/* FP:hir.rs-2971 */     AssignDesugar(Span),
/* FP:hir.rs-2972 */     /// A contract `#[ensures(..)]` attribute injects a let binding for the check that runs at point of return.
/* FP:hir.rs-2973 */     Contract,
/* FP:hir.rs-2974 */ }
/* FP:hir.rs-2975 */ 
/* FP:hir.rs-2976 */ /// Hints at the original code for a `match _ { .. }`.
/* FP:hir.rs-2977 */ #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-2978 */ pub enum MatchSource {
/* FP:hir.rs-2979 */     /// A `match _ { .. }`.
/* FP:hir.rs-2980 */     Normal,
/* FP:hir.rs-2981 */     /// A `expr.match { .. }`.
/* FP:hir.rs-2982 */     Postfix,
/* FP:hir.rs-2983 */     /// A desugared `for _ in _ { .. }` loop.
/* FP:hir.rs-2984 */     ForLoopDesugar,
/* FP:hir.rs-2985 */     /// A desugared `?` operator.
/* FP:hir.rs-2986 */     TryDesugar(HirId),
/* FP:hir.rs-2987 */     /// A desugared `<expr>.await`.
/* FP:hir.rs-2988 */     AwaitDesugar,
/* FP:hir.rs-2989 */     /// A desugared `format_args!()`.
/* FP:hir.rs-2990 */     FormatArgs,
/* FP:hir.rs-2991 */ }
/* FP:hir.rs-2992 */ 
/* FP:hir.rs-2993 */ impl MatchSource {
/* FP:hir.rs-2994 */     #[inline]
/* FP:hir.rs-2995 */     pub const fn name(self) -> &'static str {
/* FP:hir.rs-2996 */         use MatchSource::*;
/* FP:hir.rs-2997 */         match self {
/* FP:hir.rs-2998 */             Normal => "match",
/* FP:hir.rs-2999 */             Postfix => ".match",
/* FP:hir.rs-3000 */             ForLoopDesugar => "for",
/* FP:hir.rs-3001 */             TryDesugar(_) => "?",
/* FP:hir.rs-3002 */             AwaitDesugar => ".await",
/* FP:hir.rs-3003 */             FormatArgs => "format_args!()",
/* FP:hir.rs-3004 */         }
/* FP:hir.rs-3005 */     }
/* FP:hir.rs-3006 */ }
/* FP:hir.rs-3007 */ 
/* FP:hir.rs-3008 */ /// The loop type that yielded an `ExprKind::Loop`.
/* FP:hir.rs-3009 */ #[derive(Copy, Clone, PartialEq, Debug, HashStable_Generic)]
/* FP:hir.rs-3010 */ pub enum LoopSource {
/* FP:hir.rs-3011 */     /// A `loop { .. }` loop.
/* FP:hir.rs-3012 */     Loop,
/* FP:hir.rs-3013 */     /// A `while _ { .. }` loop.
/* FP:hir.rs-3014 */     While,
/* FP:hir.rs-3015 */     /// A `for _ in _ { .. }` loop.
/* FP:hir.rs-3016 */     ForLoop,
/* FP:hir.rs-3017 */ }
/* FP:hir.rs-3018 */ 
/* FP:hir.rs-3019 */ impl LoopSource {
/* FP:hir.rs-3020 */     pub fn name(self) -> &'static str {
/* FP:hir.rs-3021 */         match self {
/* FP:hir.rs-3022 */             LoopSource::Loop => "loop",
/* FP:hir.rs-3023 */             LoopSource::While => "while",
/* FP:hir.rs-3024 */             LoopSource::ForLoop => "for",
/* FP:hir.rs-3025 */         }
/* FP:hir.rs-3026 */     }
/* FP:hir.rs-3027 */ }
/* FP:hir.rs-3028 */ 
/* FP:hir.rs-3029 */ #[derive(Copy, Clone, Debug, PartialEq, HashStable_Generic)]
/* FP:hir.rs-3030 */ pub enum LoopIdError {
/* FP:hir.rs-3031 */     OutsideLoopScope,
/* FP:hir.rs-3032 */     UnlabeledCfInWhileCondition,
/* FP:hir.rs-3033 */     UnresolvedLabel,
/* FP:hir.rs-3034 */ }
/* FP:hir.rs-3035 */ 
/* FP:hir.rs-3036 */ impl fmt::Display for LoopIdError {
/* FP:hir.rs-3037 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-3038 */         f.write_str(match self {
/* FP:hir.rs-3039 */             LoopIdError::OutsideLoopScope => "not inside loop scope",
/* FP:hir.rs-3040 */             LoopIdError::UnlabeledCfInWhileCondition => {
/* FP:hir.rs-3041 */                 "unlabeled control flow (break or continue) in while condition"
/* FP:hir.rs-3042 */             }
/* FP:hir.rs-3043 */             LoopIdError::UnresolvedLabel => "label not found",
/* FP:hir.rs-3044 */         })
/* FP:hir.rs-3045 */     }
/* FP:hir.rs-3046 */ }
/* FP:hir.rs-3047 */ 
/* FP:hir.rs-3048 */ #[derive(Copy, Clone, Debug, PartialEq, HashStable_Generic)]
/* FP:hir.rs-3049 */ pub struct Destination {
/* FP:hir.rs-3050 */     /// This is `Some(_)` iff there is an explicit user-specified 'label
/* FP:hir.rs-3051 */     pub label: Option<Label>,
/* FP:hir.rs-3052 */ 
/* FP:hir.rs-3053 */     /// These errors are caught and then reported during the diagnostics pass in
/* FP:hir.rs-3054 */     /// `librustc_passes/loops.rs`
/* FP:hir.rs-3055 */     pub target_id: Result<HirId, LoopIdError>,
/* FP:hir.rs-3056 */ }
/* FP:hir.rs-3057 */ 
/* FP:hir.rs-3058 */ /// The yield kind that caused an `ExprKind::Yield`.
/* FP:hir.rs-3059 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-3060 */ pub enum YieldSource {
/* FP:hir.rs-3061 */     /// An `<expr>.await`.
/* FP:hir.rs-3062 */     Await { expr: Option<HirId> },
/* FP:hir.rs-3063 */     /// A plain `yield`.
/* FP:hir.rs-3064 */     Yield,
/* FP:hir.rs-3065 */ }
/* FP:hir.rs-3066 */ 
/* FP:hir.rs-3067 */ impl fmt::Display for YieldSource {
/* FP:hir.rs-3068 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-3069 */         f.write_str(match self {
/* FP:hir.rs-3070 */             YieldSource::Await { .. } => "`await`",
/* FP:hir.rs-3071 */             YieldSource::Yield => "`yield`",
/* FP:hir.rs-3072 */         })
/* FP:hir.rs-3073 */     }
/* FP:hir.rs-3074 */ }
/* FP:hir.rs-3075 */ 
/* FP:hir.rs-3076 */ // N.B., if you change this, you'll probably want to change the corresponding
/* FP:hir.rs-3077 */ // type structure in middle/ty.rs as well.
/* FP:hir.rs-3078 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3079 */ pub struct MutTy<'hir> {
/* FP:hir.rs-3080 */     pub ty: &'hir Ty<'hir>,
/* FP:hir.rs-3081 */     pub mutbl: Mutability,
/* FP:hir.rs-3082 */ }
/* FP:hir.rs-3083 */ 
/* FP:hir.rs-3084 */ /// Represents a function's signature in a trait declaration,
/* FP:hir.rs-3085 */ /// trait implementation, or a free function.
/* FP:hir.rs-3086 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3087 */ pub struct FnSig<'hir> {
/* FP:hir.rs-3088 */     pub header: FnHeader,
/* FP:hir.rs-3089 */     pub decl: &'hir FnDecl<'hir>,
/* FP:hir.rs-3090 */     pub span: Span,
/* FP:hir.rs-3091 */ }
/* FP:hir.rs-3092 */ 
/* FP:hir.rs-3093 */ // The bodies for items are stored "out of line", in a separate
/* FP:hir.rs-3094 */ // hashmap in the `Crate`. Here we just record the hir-id of the item
/* FP:hir.rs-3095 */ // so it can fetched later.
/* FP:hir.rs-3096 */ #[derive(Copy, Clone, PartialEq, Eq, Encodable, Decodable, Debug, HashStable_Generic)]
/* FP:hir.rs-3097 */ pub struct TraitItemId {
/* FP:hir.rs-3098 */     pub owner_id: OwnerId,
/* FP:hir.rs-3099 */ }
/* FP:hir.rs-3100 */ 
/* FP:hir.rs-3101 */ impl TraitItemId {
/* FP:hir.rs-3102 */     #[inline]
/* FP:hir.rs-3103 */     pub fn hir_id(&self) -> HirId {
/* FP:hir.rs-3104 */         // Items are always HIR owners.
/* FP:hir.rs-3105 */         HirId::make_owner(self.owner_id.def_id)
/* FP:hir.rs-3106 */     }
/* FP:hir.rs-3107 */ }
/* FP:hir.rs-3108 */ 
/* FP:hir.rs-3109 */ /// Represents an item declaration within a trait declaration,
/* FP:hir.rs-3110 */ /// possibly including a default implementation. A trait item is
/* FP:hir.rs-3111 */ /// either required (meaning it doesn't have an implementation, just a
/* FP:hir.rs-3112 */ /// signature) or provided (meaning it has a default implementation).
/* FP:hir.rs-3113 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3114 */ pub struct TraitItem<'hir> {
/* FP:hir.rs-3115 */     pub ident: Ident,
/* FP:hir.rs-3116 */     pub owner_id: OwnerId,
/* FP:hir.rs-3117 */     pub generics: &'hir Generics<'hir>,
/* FP:hir.rs-3118 */     pub kind: TraitItemKind<'hir>,
/* FP:hir.rs-3119 */     pub span: Span,
/* FP:hir.rs-3120 */     pub defaultness: Defaultness,
/* FP:hir.rs-3121 */     pub has_delayed_lints: bool,
/* FP:hir.rs-3122 */ }
/* FP:hir.rs-3123 */ 
/* FP:hir.rs-3124 */ macro_rules! expect_methods_self_kind {
/* FP:hir.rs-3125 */     ( $( $name:ident, $ret_ty:ty, $pat:pat, $ret_val:expr; )* ) => {
/* FP:hir.rs-3126 */         $(
/* FP:hir.rs-3127 */             #[track_caller]
/* FP:hir.rs-3128 */             pub fn $name(&self) -> $ret_ty {
/* FP:hir.rs-3129 */                 let $pat = &self.kind else { expect_failed(stringify!($ident), self) };
/* FP:hir.rs-3130 */                 $ret_val
/* FP:hir.rs-3131 */             }
/* FP:hir.rs-3132 */         )*
/* FP:hir.rs-3133 */     }
/* FP:hir.rs-3134 */ }
/* FP:hir.rs-3135 */ 
/* FP:hir.rs-3136 */ macro_rules! expect_methods_self {
/* FP:hir.rs-3137 */     ( $( $name:ident, $ret_ty:ty, $pat:pat, $ret_val:expr; )* ) => {
/* FP:hir.rs-3138 */         $(
/* FP:hir.rs-3139 */             #[track_caller]
/* FP:hir.rs-3140 */             pub fn $name(&self) -> $ret_ty {
/* FP:hir.rs-3141 */                 let $pat = self else { expect_failed(stringify!($ident), self) };
/* FP:hir.rs-3142 */                 $ret_val
/* FP:hir.rs-3143 */             }
/* FP:hir.rs-3144 */         )*
/* FP:hir.rs-3145 */     }
/* FP:hir.rs-3146 */ }
/* FP:hir.rs-3147 */ 
/* FP:hir.rs-3148 */ #[track_caller]
/* FP:hir.rs-3149 */ fn expect_failed<T: fmt::Debug>(ident: &'static str, found: T) -> ! {
/* FP:hir.rs-3150 */     panic!("{ident}: found {found:?}")
/* FP:hir.rs-3151 */ }
/* FP:hir.rs-3152 */ 
/* FP:hir.rs-3153 */ impl<'hir> TraitItem<'hir> {
/* FP:hir.rs-3154 */     #[inline]
/* FP:hir.rs-3155 */     pub fn hir_id(&self) -> HirId {
/* FP:hir.rs-3156 */         // Items are always HIR owners.
/* FP:hir.rs-3157 */         HirId::make_owner(self.owner_id.def_id)
/* FP:hir.rs-3158 */     }
/* FP:hir.rs-3159 */ 
/* FP:hir.rs-3160 */     pub fn trait_item_id(&self) -> TraitItemId {
/* FP:hir.rs-3161 */         TraitItemId { owner_id: self.owner_id }
/* FP:hir.rs-3162 */     }
/* FP:hir.rs-3163 */ 
/* FP:hir.rs-3164 */     expect_methods_self_kind! {
/* FP:hir.rs-3165 */         expect_const, (&'hir Ty<'hir>, Option<BodyId>),
/* FP:hir.rs-3166 */             TraitItemKind::Const(ty, body), (ty, *body);
/* FP:hir.rs-3167 */ 
/* FP:hir.rs-3168 */         expect_fn, (&FnSig<'hir>, &TraitFn<'hir>),
/* FP:hir.rs-3169 */             TraitItemKind::Fn(ty, trfn), (ty, trfn);
/* FP:hir.rs-3170 */ 
/* FP:hir.rs-3171 */         expect_type, (GenericBounds<'hir>, Option<&'hir Ty<'hir>>),
/* FP:hir.rs-3172 */             TraitItemKind::Type(bounds, ty), (bounds, *ty);
/* FP:hir.rs-3173 */     }
/* FP:hir.rs-3174 */ }
/* FP:hir.rs-3175 */ 
/* FP:hir.rs-3176 */ /// Represents a trait method's body (or just argument names).
/* FP:hir.rs-3177 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3178 */ pub enum TraitFn<'hir> {
/* FP:hir.rs-3179 */     /// No default body in the trait, just a signature.
/* FP:hir.rs-3180 */     Required(&'hir [Option<Ident>]),
/* FP:hir.rs-3181 */ 
/* FP:hir.rs-3182 */     /// Both signature and body are provided in the trait.
/* FP:hir.rs-3183 */     Provided(BodyId),
/* FP:hir.rs-3184 */ }
/* FP:hir.rs-3185 */ 
/* FP:hir.rs-3186 */ /// Represents a trait method or associated constant or type
/* FP:hir.rs-3187 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3188 */ pub enum TraitItemKind<'hir> {
/* FP:hir.rs-3189 */     /// An associated constant with an optional value (otherwise `impl`s must contain a value).
/* FP:hir.rs-3190 */     Const(&'hir Ty<'hir>, Option<BodyId>),
/* FP:hir.rs-3191 */     /// An associated function with an optional body.
/* FP:hir.rs-3192 */     Fn(FnSig<'hir>, TraitFn<'hir>),
/* FP:hir.rs-3193 */     /// An associated type with (possibly empty) bounds and optional concrete
/* FP:hir.rs-3194 */     /// type.
/* FP:hir.rs-3195 */     Type(GenericBounds<'hir>, Option<&'hir Ty<'hir>>),
/* FP:hir.rs-3196 */ }
/* FP:hir.rs-3197 */ 
/* FP:hir.rs-3198 */ // The bodies for items are stored "out of line", in a separate
/* FP:hir.rs-3199 */ // hashmap in the `Crate`. Here we just record the hir-id of the item
/* FP:hir.rs-3200 */ // so it can fetched later.
/* FP:hir.rs-3201 */ #[derive(Copy, Clone, PartialEq, Eq, Encodable, Decodable, Debug, HashStable_Generic)]
/* FP:hir.rs-3202 */ pub struct ImplItemId {
/* FP:hir.rs-3203 */     pub owner_id: OwnerId,
/* FP:hir.rs-3204 */ }
/* FP:hir.rs-3205 */ 
/* FP:hir.rs-3206 */ impl ImplItemId {
/* FP:hir.rs-3207 */     #[inline]
/* FP:hir.rs-3208 */     pub fn hir_id(&self) -> HirId {
/* FP:hir.rs-3209 */         // Items are always HIR owners.
/* FP:hir.rs-3210 */         HirId::make_owner(self.owner_id.def_id)
/* FP:hir.rs-3211 */     }
/* FP:hir.rs-3212 */ }
/* FP:hir.rs-3213 */ 
/* FP:hir.rs-3214 */ /// Represents an associated item within an impl block.
/* FP:hir.rs-3215 */ ///
/* FP:hir.rs-3216 */ /// Refer to [`Impl`] for an impl block declaration.
/* FP:hir.rs-3217 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3218 */ pub struct ImplItem<'hir> {
/* FP:hir.rs-3219 */     pub ident: Ident,
/* FP:hir.rs-3220 */     pub owner_id: OwnerId,
/* FP:hir.rs-3221 */     pub generics: &'hir Generics<'hir>,
/* FP:hir.rs-3222 */     pub kind: ImplItemKind<'hir>,
/* FP:hir.rs-3223 */     pub impl_kind: ImplItemImplKind,
/* FP:hir.rs-3224 */     pub span: Span,
/* FP:hir.rs-3225 */     pub has_delayed_lints: bool,
/* FP:hir.rs-3226 */ }
/* FP:hir.rs-3227 */ 
/* FP:hir.rs-3228 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3229 */ pub enum ImplItemImplKind {
/* FP:hir.rs-3230 */     Inherent {
/* FP:hir.rs-3231 */         vis_span: Span,
/* FP:hir.rs-3232 */     },
/* FP:hir.rs-3233 */     Trait {
/* FP:hir.rs-3234 */         defaultness: Defaultness,
/* FP:hir.rs-3235 */         /// Item in the trait that this item implements
/* FP:hir.rs-3236 */         trait_item_def_id: Result<DefId, ErrorGuaranteed>,
/* FP:hir.rs-3237 */     },
/* FP:hir.rs-3238 */ }
/* FP:hir.rs-3239 */ 
/* FP:hir.rs-3240 */ impl<'hir> ImplItem<'hir> {
/* FP:hir.rs-3241 */     #[inline]
/* FP:hir.rs-3242 */     pub fn hir_id(&self) -> HirId {
/* FP:hir.rs-3243 */         // Items are always HIR owners.
/* FP:hir.rs-3244 */         HirId::make_owner(self.owner_id.def_id)
/* FP:hir.rs-3245 */     }
/* FP:hir.rs-3246 */ 
/* FP:hir.rs-3247 */     pub fn impl_item_id(&self) -> ImplItemId {
/* FP:hir.rs-3248 */         ImplItemId { owner_id: self.owner_id }
/* FP:hir.rs-3249 */     }
/* FP:hir.rs-3250 */ 
/* FP:hir.rs-3251 */     pub fn vis_span(&self) -> Option<Span> {
/* FP:hir.rs-3252 */         match self.impl_kind {
/* FP:hir.rs-3253 */             ImplItemImplKind::Trait { .. } => None,
/* FP:hir.rs-3254 */             ImplItemImplKind::Inherent { vis_span, .. } => Some(vis_span),
/* FP:hir.rs-3255 */         }
/* FP:hir.rs-3256 */     }
/* FP:hir.rs-3257 */ 
/* FP:hir.rs-3258 */     expect_methods_self_kind! {
/* FP:hir.rs-3259 */         expect_const, (&'hir Ty<'hir>, BodyId), ImplItemKind::Const(ty, body), (ty, *body);
/* FP:hir.rs-3260 */         expect_fn,    (&FnSig<'hir>, BodyId),   ImplItemKind::Fn(ty, body),    (ty, *body);
/* FP:hir.rs-3261 */         expect_type,  &'hir Ty<'hir>,           ImplItemKind::Type(ty),        ty;
/* FP:hir.rs-3262 */     }
/* FP:hir.rs-3263 */ }
/* FP:hir.rs-3264 */ 
/* FP:hir.rs-3265 */ /// Represents various kinds of content within an `impl`.
/* FP:hir.rs-3266 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3267 */ pub enum ImplItemKind<'hir> {
/* FP:hir.rs-3268 */     /// An associated constant of the given type, set to the constant result
/* FP:hir.rs-3269 */     /// of the expression.
/* FP:hir.rs-3270 */     Const(&'hir Ty<'hir>, BodyId),
/* FP:hir.rs-3271 */     /// An associated function implementation with the given signature and body.
/* FP:hir.rs-3272 */     Fn(FnSig<'hir>, BodyId),
/* FP:hir.rs-3273 */     /// An associated type.
/* FP:hir.rs-3274 */     Type(&'hir Ty<'hir>),
/* FP:hir.rs-3275 */ }
/* FP:hir.rs-3276 */ 
/* FP:hir.rs-3277 */ /// A constraint on an associated item.
/* FP:hir.rs-3278 */ ///
/* FP:hir.rs-3279 */ /// ### Examples
/* FP:hir.rs-3280 */ ///
/* FP:hir.rs-3281 */ /// * the `A = Ty` and `B = Ty` in `Trait<A = Ty, B = Ty>`
/* FP:hir.rs-3282 */ /// * the `G<Ty> = Ty` in `Trait<G<Ty> = Ty>`
/* FP:hir.rs-3283 */ /// * the `A: Bound` in `Trait<A: Bound>`
/* FP:hir.rs-3284 */ /// * the `RetTy` in `Trait(ArgTy, ArgTy) -> RetTy`
/* FP:hir.rs-3285 */ /// * the `C = { Ct }` in `Trait<C = { Ct }>` (feature `associated_const_equality`)
/* FP:hir.rs-3286 */ /// * the `f(..): Bound` in `Trait<f(..): Bound>` (feature `return_type_notation`)
/* FP:hir.rs-3287 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3288 */ pub struct AssocItemConstraint<'hir> {
/* FP:hir.rs-3289 */     #[stable_hasher(ignore)]
/* FP:hir.rs-3290 */     pub hir_id: HirId,
/* FP:hir.rs-3291 */     pub ident: Ident,
/* FP:hir.rs-3292 */     pub gen_args: &'hir GenericArgs<'hir>,
/* FP:hir.rs-3293 */     pub kind: AssocItemConstraintKind<'hir>,
/* FP:hir.rs-3294 */     pub span: Span,
/* FP:hir.rs-3295 */ }
/* FP:hir.rs-3296 */ 
/* FP:hir.rs-3297 */ impl<'hir> AssocItemConstraint<'hir> {
/* FP:hir.rs-3298 */     /// Obtain the type on the RHS of an assoc ty equality constraint if applicable.
/* FP:hir.rs-3299 */     pub fn ty(self) -> Option<&'hir Ty<'hir>> {
/* FP:hir.rs-3300 */         match self.kind {
/* FP:hir.rs-3301 */             AssocItemConstraintKind::Equality { term: Term::Ty(ty) } => Some(ty),
/* FP:hir.rs-3302 */             _ => None,
/* FP:hir.rs-3303 */         }
/* FP:hir.rs-3304 */     }
/* FP:hir.rs-3305 */ 
/* FP:hir.rs-3306 */     /// Obtain the const on the RHS of an assoc const equality constraint if applicable.
/* FP:hir.rs-3307 */     pub fn ct(self) -> Option<&'hir ConstArg<'hir>> {
/* FP:hir.rs-3308 */         match self.kind {
/* FP:hir.rs-3309 */             AssocItemConstraintKind::Equality { term: Term::Const(ct) } => Some(ct),
/* FP:hir.rs-3310 */             _ => None,
/* FP:hir.rs-3311 */         }
/* FP:hir.rs-3312 */     }
/* FP:hir.rs-3313 */ }
/* FP:hir.rs-3314 */ 
/* FP:hir.rs-3315 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3316 */ pub enum Term<'hir> {
/* FP:hir.rs-3317 */     Ty(&'hir Ty<'hir>),
/* FP:hir.rs-3318 */     Const(&'hir ConstArg<'hir>),
/* FP:hir.rs-3319 */ }
/* FP:hir.rs-3320 */ 
/* FP:hir.rs-3321 */ impl<'hir> From<&'hir Ty<'hir>> for Term<'hir> {
/* FP:hir.rs-3322 */     fn from(ty: &'hir Ty<'hir>) -> Self {
/* FP:hir.rs-3323 */         Term::Ty(ty)
/* FP:hir.rs-3324 */     }
/* FP:hir.rs-3325 */ }
/* FP:hir.rs-3326 */ 
/* FP:hir.rs-3327 */ impl<'hir> From<&'hir ConstArg<'hir>> for Term<'hir> {
/* FP:hir.rs-3328 */     fn from(c: &'hir ConstArg<'hir>) -> Self {
/* FP:hir.rs-3329 */         Term::Const(c)
/* FP:hir.rs-3330 */     }
/* FP:hir.rs-3331 */ }
/* FP:hir.rs-3332 */ 
/* FP:hir.rs-3333 */ /// The kind of [associated item constraint][AssocItemConstraint].
/* FP:hir.rs-3334 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3335 */ pub enum AssocItemConstraintKind<'hir> {
/* FP:hir.rs-3336 */     /// An equality constraint for an associated item (e.g., `AssocTy = Ty` in `Trait<AssocTy = Ty>`).
/* FP:hir.rs-3337 */     ///
/* FP:hir.rs-3338 */     /// Also known as an *associated item binding* (we *bind* an associated item to a term).
/* FP:hir.rs-3339 */     ///
/* FP:hir.rs-3340 */     /// Furthermore, associated type equality constraints can also be referred to as *associated type
/* FP:hir.rs-3341 */     /// bindings*. Similarly with associated const equality constraints and *associated const bindings*.
/* FP:hir.rs-3342 */     Equality { term: Term<'hir> },
/* FP:hir.rs-3343 */     /// A bound on an associated type (e.g., `AssocTy: Bound` in `Trait<AssocTy: Bound>`).
/* FP:hir.rs-3344 */     Bound { bounds: &'hir [GenericBound<'hir>] },
/* FP:hir.rs-3345 */ }
/* FP:hir.rs-3346 */ 
/* FP:hir.rs-3347 */ impl<'hir> AssocItemConstraintKind<'hir> {
/* FP:hir.rs-3348 */     pub fn descr(&self) -> &'static str {
/* FP:hir.rs-3349 */         match self {
/* FP:hir.rs-3350 */             AssocItemConstraintKind::Equality { .. } => "binding",
/* FP:hir.rs-3351 */             AssocItemConstraintKind::Bound { .. } => "constraint",
/* FP:hir.rs-3352 */         }
/* FP:hir.rs-3353 */     }
/* FP:hir.rs-3354 */ }
/* FP:hir.rs-3355 */ 
/* FP:hir.rs-3356 */ /// An uninhabited enum used to make `Infer` variants on [`Ty`] and [`ConstArg`] be
/* FP:hir.rs-3357 */ /// unreachable. Zero-Variant enums are guaranteed to have the same layout as the never
/* FP:hir.rs-3358 */ /// type.
/* FP:hir.rs-3359 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3360 */ pub enum AmbigArg {}
/* FP:hir.rs-3361 */ 
/* FP:hir.rs-3362 */ /// Represents a type in the `HIR`.
/* FP:hir.rs-3363 */ ///
/* FP:hir.rs-3364 */ /// For an explanation of the `Unambig` generic parameter see the dev-guide:
/* FP:hir.rs-3365 */ /// <https://rustc-dev-guide.rust-lang.org/hir/ambig-unambig-ty-and-consts.html>
/* FP:hir.rs-3366 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3367 */ #[repr(C)]
/* FP:hir.rs-3368 */ pub struct Ty<'hir, Unambig = ()> {
/* FP:hir.rs-3369 */     #[stable_hasher(ignore)]
/* FP:hir.rs-3370 */     pub hir_id: HirId,
/* FP:hir.rs-3371 */     pub span: Span,
/* FP:hir.rs-3372 */     pub kind: TyKind<'hir, Unambig>,
/* FP:hir.rs-3373 */ }
/* FP:hir.rs-3374 */ 
/* FP:hir.rs-3375 */ impl<'hir> Ty<'hir, AmbigArg> {
/* FP:hir.rs-3376 */     /// Converts a `Ty` in an ambiguous position to one in an unambiguous position.
/* FP:hir.rs-3377 */     ///
/* FP:hir.rs-3378 */     /// Functions accepting an unambiguous types may expect the [`TyKind::Infer`] variant
/* FP:hir.rs-3379 */     /// to be used. Care should be taken to separately handle infer types when calling this
/* FP:hir.rs-3380 */     /// function as it cannot be handled by downstream code making use of the returned ty.
/* FP:hir.rs-3381 */     ///
/* FP:hir.rs-3382 */     /// In practice this may mean overriding the [`Visitor::visit_infer`][visit_infer] method on hir visitors, or
/* FP:hir.rs-3383 */     /// specifically matching on [`GenericArg::Infer`] when handling generic arguments.
/* FP:hir.rs-3384 */     ///
/* FP:hir.rs-3385 */     /// [visit_infer]: [crate::rustc_hir::intravisit::Visitor::visit_infer]
/* FP:hir.rs-3386 */     pub fn as_unambig_ty(&self) -> &Ty<'hir> {
/* FP:hir.rs-3387 */         // SAFETY: `Ty` is `repr(C)` and `TyKind` is marked `repr(u8)` so that the layout is
/* FP:hir.rs-3388 */         // the same across different ZST type arguments.
/* FP:hir.rs-3389 */         let ptr = self as *const Ty<'hir, AmbigArg> as *const Ty<'hir, ()>;
/* FP:hir.rs-3390 */         unsafe { &*ptr }
/* FP:hir.rs-3391 */     }
/* FP:hir.rs-3392 */ }
/* FP:hir.rs-3393 */ 
/* FP:hir.rs-3394 */ impl<'hir> Ty<'hir> {
/* FP:hir.rs-3395 */     /// Converts a `Ty` in an unambiguous position to one in an ambiguous position. This is
/* FP:hir.rs-3396 */     /// fallible as the [`TyKind::Infer`] variant is not present in ambiguous positions.
/* FP:hir.rs-3397 */     ///
/* FP:hir.rs-3398 */     /// Functions accepting ambiguous types will not handle the [`TyKind::Infer`] variant, if
/* FP:hir.rs-3399 */     /// infer types are relevant to you then care should be taken to handle them separately.
/* FP:hir.rs-3400 */     pub fn try_as_ambig_ty(&self) -> Option<&Ty<'hir, AmbigArg>> {
/* FP:hir.rs-3401 */         if let TyKind::Infer(()) = self.kind {
/* FP:hir.rs-3402 */             return None;
/* FP:hir.rs-3403 */         }
/* FP:hir.rs-3404 */ 
/* FP:hir.rs-3405 */         // SAFETY: `Ty` is `repr(C)` and `TyKind` is marked `repr(u8)` so that the layout is
/* FP:hir.rs-3406 */         // the same across different ZST type arguments. We also asserted that the `self` is
/* FP:hir.rs-3407 */         // not a `TyKind::Infer` so there is no risk of transmuting a `()` to `AmbigArg`.
/* FP:hir.rs-3408 */         let ptr = self as *const Ty<'hir> as *const Ty<'hir, AmbigArg>;
/* FP:hir.rs-3409 */         Some(unsafe { &*ptr })
/* FP:hir.rs-3410 */     }
/* FP:hir.rs-3411 */ }
/* FP:hir.rs-3412 */ 
/* FP:hir.rs-3413 */ impl<'hir> Ty<'hir, AmbigArg> {
/* FP:hir.rs-3414 */     pub fn peel_refs(&self) -> &Ty<'hir> {
/* FP:hir.rs-3415 */         let mut final_ty = self.as_unambig_ty();
/* FP:hir.rs-3416 */         while let TyKind::Ref(_, MutTy { ty, .. }) = &final_ty.kind {
/* FP:hir.rs-3417 */             final_ty = ty;
/* FP:hir.rs-3418 */         }
/* FP:hir.rs-3419 */         final_ty
/* FP:hir.rs-3420 */     }
/* FP:hir.rs-3421 */ }
/* FP:hir.rs-3422 */ 
/* FP:hir.rs-3423 */ impl<'hir> Ty<'hir> {
/* FP:hir.rs-3424 */     pub fn peel_refs(&self) -> &Self {
/* FP:hir.rs-3425 */         let mut final_ty = self;
/* FP:hir.rs-3426 */         while let TyKind::Ref(_, MutTy { ty, .. }) = &final_ty.kind {
/* FP:hir.rs-3427 */             final_ty = ty;
/* FP:hir.rs-3428 */         }
/* FP:hir.rs-3429 */         final_ty
/* FP:hir.rs-3430 */     }
/* FP:hir.rs-3431 */ 
/* FP:hir.rs-3432 */     /// Returns `true` if `param_def_id` matches the `bounded_ty` of this predicate.
/* FP:hir.rs-3433 */     pub fn as_generic_param(&self) -> Option<(DefId, Ident)> {
/* FP:hir.rs-3434 */         let TyKind::Path(QPath::Resolved(None, path)) = self.kind else {
/* FP:hir.rs-3435 */             return None;
/* FP:hir.rs-3436 */         };
/* FP:hir.rs-3437 */         let [segment] = &path.segments else {
/* FP:hir.rs-3438 */             return None;
/* FP:hir.rs-3439 */         };
/* FP:hir.rs-3440 */         match path.res {
/* FP:hir.rs-3441 */             Res::Def(DefKind::TyParam, def_id) | Res::SelfTyParam { trait_: def_id } => {
/* FP:hir.rs-3442 */                 Some((def_id, segment.ident))
/* FP:hir.rs-3443 */             }
/* FP:hir.rs-3444 */             _ => None,
/* FP:hir.rs-3445 */         }
/* FP:hir.rs-3446 */     }
/* FP:hir.rs-3447 */ 
/* FP:hir.rs-3448 */     pub fn find_self_aliases(&self) -> Vec<Span> {
/* FP:hir.rs-3449 */         use crate::intravisit::Visitor;
/* FP:hir.rs-3450 */         struct MyVisitor(Vec<Span>);
/* FP:hir.rs-3451 */         impl<'v> Visitor<'v> for MyVisitor {
/* FP:hir.rs-3452 */             fn visit_ty(&mut self, t: &'v Ty<'v, AmbigArg>) {
/* FP:hir.rs-3453 */                 if matches!(
/* FP:hir.rs-3454 */                     &t.kind,
/* FP:hir.rs-3455 */                     TyKind::Path(QPath::Resolved(
/* FP:hir.rs-3456 */                         _,
/* FP:hir.rs-3457 */                         Path { res: crate::def::Res::SelfTyAlias { .. }, .. },
/* FP:hir.rs-3458 */                     ))
/* FP:hir.rs-3459 */                 ) {
/* FP:hir.rs-3460 */                     self.0.push(t.span);
/* FP:hir.rs-3461 */                     return;
/* FP:hir.rs-3462 */                 }
/* FP:hir.rs-3463 */                 crate::intravisit::walk_ty(self, t);
/* FP:hir.rs-3464 */             }
/* FP:hir.rs-3465 */         }
/* FP:hir.rs-3466 */ 
/* FP:hir.rs-3467 */         let mut my_visitor = MyVisitor(vec![]);
/* FP:hir.rs-3468 */         my_visitor.visit_ty_unambig(self);
/* FP:hir.rs-3469 */         my_visitor.0
/* FP:hir.rs-3470 */     }
/* FP:hir.rs-3471 */ 
/* FP:hir.rs-3472 */     /// Whether `ty` is a type with `_` placeholders that can be inferred. Used in diagnostics only to
/* FP:hir.rs-3473 */     /// use inference to provide suggestions for the appropriate type if possible.
/* FP:hir.rs-3474 */     pub fn is_suggestable_infer_ty(&self) -> bool {
/* FP:hir.rs-3475 */         fn are_suggestable_generic_args(generic_args: &[GenericArg<'_>]) -> bool {
/* FP:hir.rs-3476 */             generic_args.iter().any(|arg| match arg {
/* FP:hir.rs-3477 */                 GenericArg::Type(ty) => ty.as_unambig_ty().is_suggestable_infer_ty(),
/* FP:hir.rs-3478 */                 GenericArg::Infer(_) => true,
/* FP:hir.rs-3479 */                 _ => false,
/* FP:hir.rs-3480 */             })
/* FP:hir.rs-3481 */         }
/* FP:hir.rs-3482 */         debug!(?self);
/* FP:hir.rs-3483 */         match &self.kind {
/* FP:hir.rs-3484 */             TyKind::Infer(()) => true,
/* FP:hir.rs-3485 */             TyKind::Slice(ty) => ty.is_suggestable_infer_ty(),
/* FP:hir.rs-3486 */             TyKind::Array(ty, length) => {
/* FP:hir.rs-3487 */                 ty.is_suggestable_infer_ty() || matches!(length.kind, ConstArgKind::Infer(..))
/* FP:hir.rs-3488 */             }
/* FP:hir.rs-3489 */             TyKind::Tup(tys) => tys.iter().any(Self::is_suggestable_infer_ty),
/* FP:hir.rs-3490 */             TyKind::Ptr(mut_ty) | TyKind::Ref(_, mut_ty) => mut_ty.ty.is_suggestable_infer_ty(),
/* FP:hir.rs-3491 */             TyKind::Path(QPath::TypeRelative(ty, segment)) => {
/* FP:hir.rs-3492 */                 ty.is_suggestable_infer_ty() || are_suggestable_generic_args(segment.args().args)
/* FP:hir.rs-3493 */             }
/* FP:hir.rs-3494 */             TyKind::Path(QPath::Resolved(ty_opt, Path { segments, .. })) => {
/* FP:hir.rs-3495 */                 ty_opt.is_some_and(Self::is_suggestable_infer_ty)
/* FP:hir.rs-3496 */                     || segments
/* FP:hir.rs-3497 */                         .iter()
/* FP:hir.rs-3498 */                         .any(|segment| are_suggestable_generic_args(segment.args().args))
/* FP:hir.rs-3499 */             }
/* FP:hir.rs-3500 */             _ => false,
/* FP:hir.rs-3501 */         }
/* FP:hir.rs-3502 */     }
/* FP:hir.rs-3503 */ }
/* FP:hir.rs-3504 */ 
/* FP:hir.rs-3505 */ /// Not represented directly in the AST; referred to by name through a `ty_path`.
/* FP:hir.rs-3506 */ #[derive(Copy, Clone, PartialEq, Eq, Encodable, Decodable, Hash, Debug, HashStable_Generic)]
/* FP:hir.rs-3507 */ pub enum PrimTy {
/* FP:hir.rs-3508 */     Int(IntTy),
/* FP:hir.rs-3509 */     Uint(UintTy),
/* FP:hir.rs-3510 */     Float(FloatTy),
/* FP:hir.rs-3511 */     Str,
/* FP:hir.rs-3512 */     Bool,
/* FP:hir.rs-3513 */     Char,
/* FP:hir.rs-3514 */ }
/* FP:hir.rs-3515 */ 
/* FP:hir.rs-3516 */ impl PrimTy {
/* FP:hir.rs-3517 */     /// All of the primitive types
/* FP:hir.rs-3518 */     pub const ALL: [Self; 19] = [
/* FP:hir.rs-3519 */         // any changes here should also be reflected in `PrimTy::from_name`
/* FP:hir.rs-3520 */         Self::Int(IntTy::I8),
/* FP:hir.rs-3521 */         Self::Int(IntTy::I16),
/* FP:hir.rs-3522 */         Self::Int(IntTy::I32),
/* FP:hir.rs-3523 */         Self::Int(IntTy::I64),
/* FP:hir.rs-3524 */         Self::Int(IntTy::I128),
/* FP:hir.rs-3525 */         Self::Int(IntTy::Isize),
/* FP:hir.rs-3526 */         Self::Uint(UintTy::U8),
/* FP:hir.rs-3527 */         Self::Uint(UintTy::U16),
/* FP:hir.rs-3528 */         Self::Uint(UintTy::U32),
/* FP:hir.rs-3529 */         Self::Uint(UintTy::U64),
/* FP:hir.rs-3530 */         Self::Uint(UintTy::U128),
/* FP:hir.rs-3531 */         Self::Uint(UintTy::Usize),
/* FP:hir.rs-3532 */         Self::Float(FloatTy::F16),
/* FP:hir.rs-3533 */         Self::Float(FloatTy::F32),
/* FP:hir.rs-3534 */         Self::Float(FloatTy::F64),
/* FP:hir.rs-3535 */         Self::Float(FloatTy::F128),
/* FP:hir.rs-3536 */         Self::Bool,
/* FP:hir.rs-3537 */         Self::Char,
/* FP:hir.rs-3538 */         Self::Str,
/* FP:hir.rs-3539 */     ];
/* FP:hir.rs-3540 */ 
/* FP:hir.rs-3541 */     /// Like [`PrimTy::name`], but returns a &str instead of a symbol.
/* FP:hir.rs-3542 */     ///
/* FP:hir.rs-3543 */     /// Used by clippy.
/* FP:hir.rs-3544 */     pub fn name_str(self) -> &'static str {
/* FP:hir.rs-3545 */         match self {
/* FP:hir.rs-3546 */             PrimTy::Int(i) => i.name_str(),
/* FP:hir.rs-3547 */             PrimTy::Uint(u) => u.name_str(),
/* FP:hir.rs-3548 */             PrimTy::Float(f) => f.name_str(),
/* FP:hir.rs-3549 */             PrimTy::Str => "str",
/* FP:hir.rs-3550 */             PrimTy::Bool => "bool",
/* FP:hir.rs-3551 */             PrimTy::Char => "char",
/* FP:hir.rs-3552 */         }
/* FP:hir.rs-3553 */     }
/* FP:hir.rs-3554 */ 
/* FP:hir.rs-3555 */     pub fn name(self) -> Symbol {
/* FP:hir.rs-3556 */         match self {
/* FP:hir.rs-3557 */             PrimTy::Int(i) => i.name(),
/* FP:hir.rs-3558 */             PrimTy::Uint(u) => u.name(),
/* FP:hir.rs-3559 */             PrimTy::Float(f) => f.name(),
/* FP:hir.rs-3560 */             PrimTy::Str => sym::str,
/* FP:hir.rs-3561 */             PrimTy::Bool => sym::bool,
/* FP:hir.rs-3562 */             PrimTy::Char => sym::char,
/* FP:hir.rs-3563 */         }
/* FP:hir.rs-3564 */     }
/* FP:hir.rs-3565 */ 
/* FP:hir.rs-3566 */     /// Returns the matching `PrimTy` for a `Symbol` such as "str" or "i32".
/* FP:hir.rs-3567 */     /// Returns `None` if no matching type is found.
/* FP:hir.rs-3568 */     pub fn from_name(name: Symbol) -> Option<Self> {
/* FP:hir.rs-3569 */         let ty = match name {
/* FP:hir.rs-3570 */             // any changes here should also be reflected in `PrimTy::ALL`
/* FP:hir.rs-3571 */             sym::i8 => Self::Int(IntTy::I8),
/* FP:hir.rs-3572 */             sym::i16 => Self::Int(IntTy::I16),
/* FP:hir.rs-3573 */             sym::i32 => Self::Int(IntTy::I32),
/* FP:hir.rs-3574 */             sym::i64 => Self::Int(IntTy::I64),
/* FP:hir.rs-3575 */             sym::i128 => Self::Int(IntTy::I128),
/* FP:hir.rs-3576 */             sym::isize => Self::Int(IntTy::Isize),
/* FP:hir.rs-3577 */             sym::u8 => Self::Uint(UintTy::U8),
/* FP:hir.rs-3578 */             sym::u16 => Self::Uint(UintTy::U16),
/* FP:hir.rs-3579 */             sym::u32 => Self::Uint(UintTy::U32),
/* FP:hir.rs-3580 */             sym::u64 => Self::Uint(UintTy::U64),
/* FP:hir.rs-3581 */             sym::u128 => Self::Uint(UintTy::U128),
/* FP:hir.rs-3582 */             sym::usize => Self::Uint(UintTy::Usize),
/* FP:hir.rs-3583 */             sym::f16 => Self::Float(FloatTy::F16),
/* FP:hir.rs-3584 */             sym::f32 => Self::Float(FloatTy::F32),
/* FP:hir.rs-3585 */             sym::f64 => Self::Float(FloatTy::F64),
/* FP:hir.rs-3586 */             sym::f128 => Self::Float(FloatTy::F128),
/* FP:hir.rs-3587 */             sym::bool => Self::Bool,
/* FP:hir.rs-3588 */             sym::char => Self::Char,
/* FP:hir.rs-3589 */             sym::str => Self::Str,
/* FP:hir.rs-3590 */             _ => return None,
/* FP:hir.rs-3591 */         };
/* FP:hir.rs-3592 */         Some(ty)
/* FP:hir.rs-3593 */     }
/* FP:hir.rs-3594 */ }
/* FP:hir.rs-3595 */ 
/* FP:hir.rs-3596 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3597 */ pub struct FnPtrTy<'hir> {
/* FP:hir.rs-3598 */     pub safety: Safety,
/* FP:hir.rs-3599 */     pub abi: ExternAbi,
/* FP:hir.rs-3600 */     pub generic_params: &'hir [GenericParam<'hir>],
/* FP:hir.rs-3601 */     pub decl: &'hir FnDecl<'hir>,
/* FP:hir.rs-3602 */     // `Option` because bare fn parameter identifiers are optional. We also end up
/* FP:hir.rs-3603 */     // with `None` in some error cases, e.g. invalid parameter patterns.
/* FP:hir.rs-3604 */     pub param_idents: &'hir [Option<Ident>],
/* FP:hir.rs-3605 */ }
/* FP:hir.rs-3606 */ 
/* FP:hir.rs-3607 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3608 */ pub struct UnsafeBinderTy<'hir> {
/* FP:hir.rs-3609 */     pub generic_params: &'hir [GenericParam<'hir>],
/* FP:hir.rs-3610 */     pub inner_ty: &'hir Ty<'hir>,
/* FP:hir.rs-3611 */ }
/* FP:hir.rs-3612 */ 
/* FP:hir.rs-3613 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3614 */ pub struct OpaqueTy<'hir> {
/* FP:hir.rs-3615 */     #[stable_hasher(ignore)]
/* FP:hir.rs-3616 */     pub hir_id: HirId,
/* FP:hir.rs-3617 */     pub def_id: LocalDefId,
/* FP:hir.rs-3618 */     pub bounds: GenericBounds<'hir>,
/* FP:hir.rs-3619 */     pub origin: OpaqueTyOrigin<LocalDefId>,
/* FP:hir.rs-3620 */     pub span: Span,
/* FP:hir.rs-3621 */ }
/* FP:hir.rs-3622 */ 
/* FP:hir.rs-3623 */ #[derive(Debug, Clone, Copy, HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-3624 */ pub enum PreciseCapturingArgKind<T, U> {
/* FP:hir.rs-3625 */     Lifetime(T),
/* FP:hir.rs-3626 */     /// Non-lifetime argument (type or const)
/* FP:hir.rs-3627 */     Param(U),
/* FP:hir.rs-3628 */ }
/* FP:hir.rs-3629 */ 
/* FP:hir.rs-3630 */ pub type PreciseCapturingArg<'hir> =
/* FP:hir.rs-3631 */     PreciseCapturingArgKind<&'hir Lifetime, PreciseCapturingNonLifetimeArg>;
/* FP:hir.rs-3632 */ 
/* FP:hir.rs-3633 */ impl PreciseCapturingArg<'_> {
/* FP:hir.rs-3634 */     pub fn hir_id(self) -> HirId {
/* FP:hir.rs-3635 */         match self {
/* FP:hir.rs-3636 */             PreciseCapturingArg::Lifetime(lt) => lt.hir_id,
/* FP:hir.rs-3637 */             PreciseCapturingArg::Param(param) => param.hir_id,
/* FP:hir.rs-3638 */         }
/* FP:hir.rs-3639 */     }
/* FP:hir.rs-3640 */ 
/* FP:hir.rs-3641 */     pub fn name(self) -> Symbol {
/* FP:hir.rs-3642 */         match self {
/* FP:hir.rs-3643 */             PreciseCapturingArg::Lifetime(lt) => lt.ident.name,
/* FP:hir.rs-3644 */             PreciseCapturingArg::Param(param) => param.ident.name,
/* FP:hir.rs-3645 */         }
/* FP:hir.rs-3646 */     }
/* FP:hir.rs-3647 */ }
/* FP:hir.rs-3648 */ 
/* FP:hir.rs-3649 */ /// We need to have a [`Node`] for the [`HirId`] that we attach the type/const param
/* FP:hir.rs-3650 */ /// resolution to. Lifetimes don't have this problem, and for them, it's actually
/* FP:hir.rs-3651 */ /// kind of detrimental to use a custom node type versus just using [`Lifetime`],
/* FP:hir.rs-3652 */ /// since resolve_bound_vars operates on `Lifetime`s.
/* FP:hir.rs-3653 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3654 */ pub struct PreciseCapturingNonLifetimeArg {
/* FP:hir.rs-3655 */     #[stable_hasher(ignore)]
/* FP:hir.rs-3656 */     pub hir_id: HirId,
/* FP:hir.rs-3657 */     pub ident: Ident,
/* FP:hir.rs-3658 */     pub res: Res,
/* FP:hir.rs-3659 */ }
/* FP:hir.rs-3660 */ 
/* FP:hir.rs-3661 */ #[derive(Copy, Clone, PartialEq, Eq, Debug)]
/* FP:hir.rs-3662 */ #[derive(HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-3663 */ pub enum RpitContext {
/* FP:hir.rs-3664 */     Trait,
/* FP:hir.rs-3665 */     TraitImpl,
/* FP:hir.rs-3666 */ }
/* FP:hir.rs-3667 */ 
/* FP:hir.rs-3668 */ /// From whence the opaque type came.
/* FP:hir.rs-3669 */ #[derive(Copy, Clone, PartialEq, Eq, Debug)]
/* FP:hir.rs-3670 */ #[derive(HashStable_Generic, Encodable, Decodable)]
/* FP:hir.rs-3671 */ pub enum OpaqueTyOrigin<D> {
/* FP:hir.rs-3672 */     /// `-> impl Trait`
/* FP:hir.rs-3673 */     FnReturn {
/* FP:hir.rs-3674 */         /// The defining function.
/* FP:hir.rs-3675 */         parent: D,
/* FP:hir.rs-3676 */         // Whether this is an RPITIT (return position impl trait in trait)
/* FP:hir.rs-3677 */         in_trait_or_impl: Option<RpitContext>,
/* FP:hir.rs-3678 */     },
/* FP:hir.rs-3679 */     /// `async fn`
/* FP:hir.rs-3680 */     AsyncFn {
/* FP:hir.rs-3681 */         /// The defining function.
/* FP:hir.rs-3682 */         parent: D,
/* FP:hir.rs-3683 */         // Whether this is an AFIT (async fn in trait)
/* FP:hir.rs-3684 */         in_trait_or_impl: Option<RpitContext>,
/* FP:hir.rs-3685 */     },
/* FP:hir.rs-3686 */     /// type aliases: `type Foo = impl Trait;`
/* FP:hir.rs-3687 */     TyAlias {
/* FP:hir.rs-3688 */         /// The type alias or associated type parent of the TAIT/ATPIT
/* FP:hir.rs-3689 */         parent: D,
/* FP:hir.rs-3690 */         /// associated types in impl blocks for traits.
/* FP:hir.rs-3691 */         in_assoc_ty: bool,
/* FP:hir.rs-3692 */     },
/* FP:hir.rs-3693 */ }
/* FP:hir.rs-3694 */ 
/* FP:hir.rs-3695 */ #[derive(Debug, Clone, Copy, PartialEq, Eq, HashStable_Generic)]
/* FP:hir.rs-3696 */ pub enum InferDelegationKind {
/* FP:hir.rs-3697 */     Input(usize),
/* FP:hir.rs-3698 */     Output,
/* FP:hir.rs-3699 */ }
/* FP:hir.rs-3700 */ 
/* FP:hir.rs-3701 */ /// The various kinds of types recognized by the compiler.
/* FP:hir.rs-3702 */ ///
/* FP:hir.rs-3703 */ /// For an explanation of the `Unambig` generic parameter see the dev-guide:
/* FP:hir.rs-3704 */ /// <https://rustc-dev-guide.rust-lang.org/hir/ambig-unambig-ty-and-consts.html>
/* FP:hir.rs-3705 */ // SAFETY: `repr(u8)` is required so that `TyKind<()>` and `TyKind<!>` are layout compatible
/* FP:hir.rs-3706 */ #[repr(u8, C)]
/* FP:hir.rs-3707 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3708 */ pub enum TyKind<'hir, Unambig = ()> {
/* FP:hir.rs-3709 */     /// Actual type should be inherited from `DefId` signature
/* FP:hir.rs-3710 */     InferDelegation(DefId, InferDelegationKind),
/* FP:hir.rs-3711 */     /// A variable length slice (i.e., `[T]`).
/* FP:hir.rs-3712 */     Slice(&'hir Ty<'hir>),
/* FP:hir.rs-3713 */     /// A fixed length array (i.e., `[T; n]`).
/* FP:hir.rs-3714 */     Array(&'hir Ty<'hir>, &'hir ConstArg<'hir>),
/* FP:hir.rs-3715 */     /// A raw pointer (i.e., `*const T` or `*mut T`).
/* FP:hir.rs-3716 */     Ptr(MutTy<'hir>),
/* FP:hir.rs-3717 */     /// A reference (i.e., `&'a T` or `&'a mut T`).
/* FP:hir.rs-3718 */     Ref(&'hir Lifetime, MutTy<'hir>),
/* FP:hir.rs-3719 */     /// A function pointer (e.g., `fn(usize) -> bool`).
/* FP:hir.rs-3720 */     FnPtr(&'hir FnPtrTy<'hir>),
/* FP:hir.rs-3721 */     /// An unsafe binder type (e.g. `unsafe<'a> Foo<'a>`).
/* FP:hir.rs-3722 */     UnsafeBinder(&'hir UnsafeBinderTy<'hir>),
/* FP:hir.rs-3723 */     /// The never type (`!`).
/* FP:hir.rs-3724 */     Never,
/* FP:hir.rs-3725 */     /// A tuple (`(A, B, C, D, ...)`).
/* FP:hir.rs-3726 */     Tup(&'hir [Ty<'hir>]),
/* FP:hir.rs-3727 */     /// A path to a type definition (`module::module::...::Type`), or an
/* FP:hir.rs-3728 */     /// associated type (e.g., `<Vec<T> as Trait>::Type` or `<T>::Target`).
/* FP:hir.rs-3729 */     ///
/* FP:hir.rs-3730 */     /// Type parameters may be stored in each `PathSegment`.
/* FP:hir.rs-3731 */     Path(QPath<'hir>),
/* FP:hir.rs-3732 */     /// An opaque type definition itself. This is only used for `impl Trait`.
/* FP:hir.rs-3733 */     OpaqueDef(&'hir OpaqueTy<'hir>),
/* FP:hir.rs-3734 */     /// A trait ascription type, which is `impl Trait` within a local binding.
/* FP:hir.rs-3735 */     TraitAscription(GenericBounds<'hir>),
/* FP:hir.rs-3736 */     /// A trait object type `Bound1 + Bound2 + Bound3`
/* FP:hir.rs-3737 */     /// where `Bound` is a trait or a lifetime.
/* FP:hir.rs-3738 */     ///
/* FP:hir.rs-3739 */     /// We use pointer tagging to represent a `&'hir Lifetime` and `TraitObjectSyntax` pair
/* FP:hir.rs-3740 */     /// as otherwise this type being `repr(C)` would result in `TyKind` increasing in size.
/* FP:hir.rs-3741 */     TraitObject(&'hir [PolyTraitRef<'hir>], TaggedRef<'hir, Lifetime, TraitObjectSyntax>),
/* FP:hir.rs-3742 */     /// Unused for now.
/* FP:hir.rs-3743 */     Typeof(&'hir AnonConst),
/* FP:hir.rs-3744 */     /// Placeholder for a type that has failed to be defined.
/* FP:hir.rs-3745 */     Err(crate::rustc_span::ErrorGuaranteed),
/* FP:hir.rs-3746 */     /// Pattern types (`pattern_type!(u32 is 1..)`)
/* FP:hir.rs-3747 */     Pat(&'hir Ty<'hir>, &'hir TyPat<'hir>),
/* FP:hir.rs-3748 */     /// `TyKind::Infer` means the type should be inferred instead of it having been
/* FP:hir.rs-3749 */     /// specified. This can appear anywhere in a type.
/* FP:hir.rs-3750 */     ///
/* FP:hir.rs-3751 */     /// This variant is not always used to represent inference types, sometimes
/* FP:hir.rs-3752 */     /// [`GenericArg::Infer`] is used instead.
/* FP:hir.rs-3753 */     Infer(Unambig),
/* FP:hir.rs-3754 */ }
/* FP:hir.rs-3755 */ 
/* FP:hir.rs-3756 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3757 */ pub enum InlineAsmOperand<'hir> {
/* FP:hir.rs-3758 */     In {
/* FP:hir.rs-3759 */         reg: InlineAsmRegOrRegClass,
/* FP:hir.rs-3760 */         expr: &'hir Expr<'hir>,
/* FP:hir.rs-3761 */     },
/* FP:hir.rs-3762 */     Out {
/* FP:hir.rs-3763 */         reg: InlineAsmRegOrRegClass,
/* FP:hir.rs-3764 */         late: bool,
/* FP:hir.rs-3765 */         expr: Option<&'hir Expr<'hir>>,
/* FP:hir.rs-3766 */     },
/* FP:hir.rs-3767 */     InOut {
/* FP:hir.rs-3768 */         reg: InlineAsmRegOrRegClass,
/* FP:hir.rs-3769 */         late: bool,
/* FP:hir.rs-3770 */         expr: &'hir Expr<'hir>,
/* FP:hir.rs-3771 */     },
/* FP:hir.rs-3772 */     SplitInOut {
/* FP:hir.rs-3773 */         reg: InlineAsmRegOrRegClass,
/* FP:hir.rs-3774 */         late: bool,
/* FP:hir.rs-3775 */         in_expr: &'hir Expr<'hir>,
/* FP:hir.rs-3776 */         out_expr: Option<&'hir Expr<'hir>>,
/* FP:hir.rs-3777 */     },
/* FP:hir.rs-3778 */     Const {
/* FP:hir.rs-3779 */         anon_const: ConstBlock,
/* FP:hir.rs-3780 */     },
/* FP:hir.rs-3781 */     SymFn {
/* FP:hir.rs-3782 */         expr: &'hir Expr<'hir>,
/* FP:hir.rs-3783 */     },
/* FP:hir.rs-3784 */     SymStatic {
/* FP:hir.rs-3785 */         path: QPath<'hir>,
/* FP:hir.rs-3786 */         def_id: DefId,
/* FP:hir.rs-3787 */     },
/* FP:hir.rs-3788 */     Label {
/* FP:hir.rs-3789 */         block: &'hir Block<'hir>,
/* FP:hir.rs-3790 */     },
/* FP:hir.rs-3791 */ }
/* FP:hir.rs-3792 */ 
/* FP:hir.rs-3793 */ impl<'hir> InlineAsmOperand<'hir> {
/* FP:hir.rs-3794 */     pub fn reg(&self) -> Option<InlineAsmRegOrRegClass> {
/* FP:hir.rs-3795 */         match *self {
/* FP:hir.rs-3796 */             Self::In { reg, .. }
/* FP:hir.rs-3797 */             | Self::Out { reg, .. }
/* FP:hir.rs-3798 */             | Self::InOut { reg, .. }
/* FP:hir.rs-3799 */             | Self::SplitInOut { reg, .. } => Some(reg),
/* FP:hir.rs-3800 */             Self::Const { .. }
/* FP:hir.rs-3801 */             | Self::SymFn { .. }
/* FP:hir.rs-3802 */             | Self::SymStatic { .. }
/* FP:hir.rs-3803 */             | Self::Label { .. } => None,
/* FP:hir.rs-3804 */         }
/* FP:hir.rs-3805 */     }
/* FP:hir.rs-3806 */ 
/* FP:hir.rs-3807 */     pub fn is_clobber(&self) -> bool {
/* FP:hir.rs-3808 */         matches!(
/* FP:hir.rs-3809 */             self,
/* FP:hir.rs-3810 */             InlineAsmOperand::Out { reg: InlineAsmRegOrRegClass::Reg(_), late: _, expr: None }
/* FP:hir.rs-3811 */         )
/* FP:hir.rs-3812 */     }
/* FP:hir.rs-3813 */ }
/* FP:hir.rs-3814 */ 
/* FP:hir.rs-3815 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3816 */ pub struct InlineAsm<'hir> {
/* FP:hir.rs-3817 */     pub asm_macro: ast::AsmMacro,
/* FP:hir.rs-3818 */     pub template: &'hir [InlineAsmTemplatePiece],
/* FP:hir.rs-3819 */     pub template_strs: &'hir [(Symbol, Option<Symbol>, Span)],
/* FP:hir.rs-3820 */     pub operands: &'hir [(InlineAsmOperand<'hir>, Span)],
/* FP:hir.rs-3821 */     pub options: InlineAsmOptions,
/* FP:hir.rs-3822 */     pub line_spans: &'hir [Span],
/* FP:hir.rs-3823 */ }
/* FP:hir.rs-3824 */ 
/* FP:hir.rs-3825 */ impl InlineAsm<'_> {
/* FP:hir.rs-3826 */     pub fn contains_label(&self) -> bool {
/* FP:hir.rs-3827 */         self.operands.iter().any(|x| matches!(x.0, InlineAsmOperand::Label { .. }))
/* FP:hir.rs-3828 */     }
/* FP:hir.rs-3829 */ }
/* FP:hir.rs-3830 */ 
/* FP:hir.rs-3831 */ /// Represents a parameter in a function header.
/* FP:hir.rs-3832 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3833 */ pub struct Param<'hir> {
/* FP:hir.rs-3834 */     #[stable_hasher(ignore)]
/* FP:hir.rs-3835 */     pub hir_id: HirId,
/* FP:hir.rs-3836 */     pub pat: &'hir Pat<'hir>,
/* FP:hir.rs-3837 */     pub ty_span: Span,
/* FP:hir.rs-3838 */     pub span: Span,
/* FP:hir.rs-3839 */ }
/* FP:hir.rs-3840 */ 
/* FP:hir.rs-3841 */ /// Represents the header (not the body) of a function declaration.
/* FP:hir.rs-3842 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3843 */ pub struct FnDecl<'hir> {
/* FP:hir.rs-3844 */     /// The types of the function's parameters.
/* FP:hir.rs-3845 */     ///
/* FP:hir.rs-3846 */     /// Additional argument data is stored in the function's [body](Body::params).
/* FP:hir.rs-3847 */     pub inputs: &'hir [Ty<'hir>],
/* FP:hir.rs-3848 */     pub output: FnRetTy<'hir>,
/* FP:hir.rs-3849 */     pub c_variadic: bool,
/* FP:hir.rs-3850 */     /// Does the function have an implicit self?
/* FP:hir.rs-3851 */     pub implicit_self: ImplicitSelfKind,
/* FP:hir.rs-3852 */     /// Is lifetime elision allowed.
/* FP:hir.rs-3853 */     pub lifetime_elision_allowed: bool,
/* FP:hir.rs-3854 */ }
/* FP:hir.rs-3855 */ 
/* FP:hir.rs-3856 */ impl<'hir> FnDecl<'hir> {
/* FP:hir.rs-3857 */     pub fn opt_delegation_sig_id(&self) -> Option<DefId> {
/* FP:hir.rs-3858 */         if let FnRetTy::Return(ty) = self.output
/* FP:hir.rs-3859 */             && let TyKind::InferDelegation(sig_id, _) = ty.kind
/* FP:hir.rs-3860 */         {
/* FP:hir.rs-3861 */             return Some(sig_id);
/* FP:hir.rs-3862 */         }
/* FP:hir.rs-3863 */         None
/* FP:hir.rs-3864 */     }
/* FP:hir.rs-3865 */ }
/* FP:hir.rs-3866 */ 
/* FP:hir.rs-3867 */ /// Represents what type of implicit self a function has, if any.
/* FP:hir.rs-3868 */ #[derive(Copy, Clone, PartialEq, Eq, Encodable, Decodable, Debug, HashStable_Generic)]
/* FP:hir.rs-3869 */ pub enum ImplicitSelfKind {
/* FP:hir.rs-3870 */     /// Represents a `fn x(self);`.
/* FP:hir.rs-3871 */     Imm,
/* FP:hir.rs-3872 */     /// Represents a `fn x(mut self);`.
/* FP:hir.rs-3873 */     Mut,
/* FP:hir.rs-3874 */     /// Represents a `fn x(&self);`.
/* FP:hir.rs-3875 */     RefImm,
/* FP:hir.rs-3876 */     /// Represents a `fn x(&mut self);`.
/* FP:hir.rs-3877 */     RefMut,
/* FP:hir.rs-3878 */     /// Represents when a function does not have a self argument or
/* FP:hir.rs-3879 */     /// when a function has a `self: X` argument.
/* FP:hir.rs-3880 */     None,
/* FP:hir.rs-3881 */ }
/* FP:hir.rs-3882 */ 
/* FP:hir.rs-3883 */ impl ImplicitSelfKind {
/* FP:hir.rs-3884 */     /// Does this represent an implicit self?
/* FP:hir.rs-3885 */     pub fn has_implicit_self(&self) -> bool {
/* FP:hir.rs-3886 */         !matches!(*self, ImplicitSelfKind::None)
/* FP:hir.rs-3887 */     }
/* FP:hir.rs-3888 */ }
/* FP:hir.rs-3889 */ 
/* FP:hir.rs-3890 */ #[derive(Copy, Clone, PartialEq, Eq, Encodable, Decodable, Debug, HashStable_Generic)]
/* FP:hir.rs-3891 */ pub enum IsAsync {
/* FP:hir.rs-3892 */     Async(Span),
/* FP:hir.rs-3893 */     NotAsync,
/* FP:hir.rs-3894 */ }
/* FP:hir.rs-3895 */ 
/* FP:hir.rs-3896 */ impl IsAsync {
/* FP:hir.rs-3897 */     pub fn is_async(self) -> bool {
/* FP:hir.rs-3898 */         matches!(self, IsAsync::Async(_))
/* FP:hir.rs-3899 */     }
/* FP:hir.rs-3900 */ }
/* FP:hir.rs-3901 */ 
/* FP:hir.rs-3902 */ #[derive(Copy, Clone, PartialEq, Eq, Debug, Encodable, Decodable, HashStable_Generic)]
/* FP:hir.rs-3903 */ pub enum Defaultness {
/* FP:hir.rs-3904 */     Default { has_value: bool },
/* FP:hir.rs-3905 */     Final,
/* FP:hir.rs-3906 */ }
/* FP:hir.rs-3907 */ 
/* FP:hir.rs-3908 */ impl Defaultness {
/* FP:hir.rs-3909 */     pub fn has_value(&self) -> bool {
/* FP:hir.rs-3910 */         match *self {
/* FP:hir.rs-3911 */             Defaultness::Default { has_value } => has_value,
/* FP:hir.rs-3912 */             Defaultness::Final => true,
/* FP:hir.rs-3913 */         }
/* FP:hir.rs-3914 */     }
/* FP:hir.rs-3915 */ 
/* FP:hir.rs-3916 */     pub fn is_final(&self) -> bool {
/* FP:hir.rs-3917 */         *self == Defaultness::Final
/* FP:hir.rs-3918 */     }
/* FP:hir.rs-3919 */ 
/* FP:hir.rs-3920 */     pub fn is_default(&self) -> bool {
/* FP:hir.rs-3921 */         matches!(*self, Defaultness::Default { .. })
/* FP:hir.rs-3922 */     }
/* FP:hir.rs-3923 */ }
/* FP:hir.rs-3924 */ 
/* FP:hir.rs-3925 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3926 */ pub enum FnRetTy<'hir> {
/* FP:hir.rs-3927 */     /// Return type is not specified.
/* FP:hir.rs-3928 */     ///
/* FP:hir.rs-3929 */     /// Functions default to `()` and
/* FP:hir.rs-3930 */     /// closures default to inference. Span points to where return
/* FP:hir.rs-3931 */     /// type would be inserted.
/* FP:hir.rs-3932 */     DefaultReturn(Span),
/* FP:hir.rs-3933 */     /// Everything else.
/* FP:hir.rs-3934 */     Return(&'hir Ty<'hir>),
/* FP:hir.rs-3935 */ }
/* FP:hir.rs-3936 */ 
/* FP:hir.rs-3937 */ impl<'hir> FnRetTy<'hir> {
/* FP:hir.rs-3938 */     #[inline]
/* FP:hir.rs-3939 */     pub fn span(&self) -> Span {
/* FP:hir.rs-3940 */         match *self {
/* FP:hir.rs-3941 */             Self::DefaultReturn(span) => span,
/* FP:hir.rs-3942 */             Self::Return(ref ty) => ty.span,
/* FP:hir.rs-3943 */         }
/* FP:hir.rs-3944 */     }
/* FP:hir.rs-3945 */ 
/* FP:hir.rs-3946 */     pub fn is_suggestable_infer_ty(&self) -> Option<&'hir Ty<'hir>> {
/* FP:hir.rs-3947 */         if let Self::Return(ty) = self
/* FP:hir.rs-3948 */             && ty.is_suggestable_infer_ty()
/* FP:hir.rs-3949 */         {
/* FP:hir.rs-3950 */             return Some(*ty);
/* FP:hir.rs-3951 */         }
/* FP:hir.rs-3952 */         None
/* FP:hir.rs-3953 */     }
/* FP:hir.rs-3954 */ }
/* FP:hir.rs-3955 */ 
/* FP:hir.rs-3956 */ /// Represents `for<...>` binder before a closure
/* FP:hir.rs-3957 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-3958 */ pub enum ClosureBinder {
/* FP:hir.rs-3959 */     /// Binder is not specified.
/* FP:hir.rs-3960 */     Default,
/* FP:hir.rs-3961 */     /// Binder is specified.
/* FP:hir.rs-3962 */     ///
/* FP:hir.rs-3963 */     /// Span points to the whole `for<...>`.
/* FP:hir.rs-3964 */     For { span: Span },
/* FP:hir.rs-3965 */ }
/* FP:hir.rs-3966 */ 
/* FP:hir.rs-3967 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3968 */ pub struct Mod<'hir> {
/* FP:hir.rs-3969 */     pub spans: ModSpans,
/* FP:hir.rs-3970 */     pub item_ids: &'hir [ItemId],
/* FP:hir.rs-3971 */ }
/* FP:hir.rs-3972 */ 
/* FP:hir.rs-3973 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-3974 */ pub struct ModSpans {
/* FP:hir.rs-3975 */     /// A span from the first token past `{` to the last token until `}`.
/* FP:hir.rs-3977 */     /// to the last token in the external file.
/* FP:hir.rs-3978 */     pub inner_span: Span,
/* FP:hir.rs-3979 */     pub inject_use_span: Span,
/* FP:hir.rs-3980 */ }
/* FP:hir.rs-3981 */ 
/* FP:hir.rs-3982 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3983 */ pub struct EnumDef<'hir> {
/* FP:hir.rs-3984 */     pub variants: &'hir [Variant<'hir>],
/* FP:hir.rs-3985 */ }
/* FP:hir.rs-3986 */ 
/* FP:hir.rs-3987 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-3988 */ pub struct Variant<'hir> {
/* FP:hir.rs-3989 */     /// Name of the variant.
/* FP:hir.rs-3990 */     pub ident: Ident,
/* FP:hir.rs-3991 */     /// Id of the variant (not the constructor, see `VariantData::ctor_hir_id()`).
/* FP:hir.rs-3992 */     #[stable_hasher(ignore)]
/* FP:hir.rs-3993 */     pub hir_id: HirId,
/* FP:hir.rs-3994 */     pub def_id: LocalDefId,
/* FP:hir.rs-3995 */     /// Fields and constructor id of the variant.
/* FP:hir.rs-3996 */     pub data: VariantData<'hir>,
/* FP:hir.rs-3997 */     /// Explicit discriminant (e.g., `Foo = 1`).
/* FP:hir.rs-3998 */     pub disr_expr: Option<&'hir AnonConst>,
/* FP:hir.rs-3999 */     /// Span
/* FP:hir.rs-4000 */     pub span: Span,
/* FP:hir.rs-4001 */ }
/* FP:hir.rs-4002 */ 
/* FP:hir.rs-4003 */ #[derive(Copy, Clone, PartialEq, Debug, HashStable_Generic)]
/* FP:hir.rs-4004 */ pub enum UseKind {
/* FP:hir.rs-4005 */     /// One import, e.g., `use foo::bar` or `use foo::bar as baz`.
/* FP:hir.rs-4006 */     /// Also produced for each element of a list `use`, e.g.
/* FP:hir.rs-4007 */     /// `use foo::{a, b}` lowers to `use foo::a; use foo::b;`.
/* FP:hir.rs-4008 */     ///
/* FP:hir.rs-4009 */     /// The identifier is the name defined by the import. E.g. for `use
/* FP:hir.rs-4010 */     /// foo::bar` it is `bar`, for `use foo::bar as baz` it is `baz`.
/* FP:hir.rs-4011 */     Single(Ident),
/* FP:hir.rs-4012 */ 
/* FP:hir.rs-4013 */     /// Glob import, e.g., `use foo::*`.
/* FP:hir.rs-4014 */     Glob,
/* FP:hir.rs-4015 */ 
/* FP:hir.rs-4016 */     /// Degenerate list import, e.g., `use foo::{a, b}` produces
/* FP:hir.rs-4017 */     /// an additional `use foo::{}` for performing checks such as
/* FP:hir.rs-4018 */     /// unstable feature gating. May be removed in the future.
/* FP:hir.rs-4019 */     ListStem,
/* FP:hir.rs-4020 */ }
/* FP:hir.rs-4021 */ 
/* FP:hir.rs-4022 */ /// References to traits in impls.
/* FP:hir.rs-4023 */ ///
/* FP:hir.rs-4024 */ /// `resolve` maps each `TraitRef`'s `ref_id` to its defining trait; that's all
/* FP:hir.rs-4025 */ /// that the `ref_id` is for. Note that `ref_id`'s value is not the `HirId` of the
/* FP:hir.rs-4026 */ /// trait being referred to but just a unique `HirId` that serves as a key
/* FP:hir.rs-4027 */ /// within the resolution map.
/* FP:hir.rs-4028 */ #[derive(Clone, Debug, Copy, HashStable_Generic)]
/* FP:hir.rs-4029 */ pub struct TraitRef<'hir> {
/* FP:hir.rs-4030 */     pub path: &'hir Path<'hir>,
/* FP:hir.rs-4031 */     // Don't hash the `ref_id`. It is tracked via the thing it is used to access.
/* FP:hir.rs-4032 */     #[stable_hasher(ignore)]
/* FP:hir.rs-4033 */     pub hir_ref_id: HirId,
/* FP:hir.rs-4034 */ }
/* FP:hir.rs-4035 */ 
/* FP:hir.rs-4036 */ impl TraitRef<'_> {
/* FP:hir.rs-4037 */     /// Gets the `DefId` of the referenced trait. It _must_ actually be a trait or trait alias.
/* FP:hir.rs-4038 */     pub fn trait_def_id(&self) -> Option<DefId> {
/* FP:hir.rs-4039 */         match self.path.res {
/* FP:hir.rs-4040 */             Res::Def(DefKind::Trait | DefKind::TraitAlias, did) => Some(did),
/* FP:hir.rs-4041 */             Res::Err => None,
/* FP:hir.rs-4042 */             res => panic!("{res:?} did not resolve to a trait or trait alias"),
/* FP:hir.rs-4043 */         }
/* FP:hir.rs-4044 */     }
/* FP:hir.rs-4045 */ }
/* FP:hir.rs-4046 */ 
/* FP:hir.rs-4047 */ #[derive(Clone, Debug, Copy, HashStable_Generic)]
/* FP:hir.rs-4048 */ pub struct PolyTraitRef<'hir> {
/* FP:hir.rs-4049 */     /// The `'a` in `for<'a> Foo<&'a T>`.
/* FP:hir.rs-4050 */     pub bound_generic_params: &'hir [GenericParam<'hir>],
/* FP:hir.rs-4051 */ 
/* FP:hir.rs-4052 */     /// The constness and polarity of the trait ref.
/* FP:hir.rs-4053 */     ///
/* FP:hir.rs-4054 */     /// The `async` modifier is lowered directly into a different trait for now.
/* FP:hir.rs-4055 */     pub modifiers: TraitBoundModifiers,
/* FP:hir.rs-4056 */ 
/* FP:hir.rs-4057 */     /// The `Foo<&'a T>` in `for<'a> Foo<&'a T>`.
/* FP:hir.rs-4058 */     pub trait_ref: TraitRef<'hir>,
/* FP:hir.rs-4059 */ 
/* FP:hir.rs-4060 */     pub span: Span,
/* FP:hir.rs-4061 */ }
/* FP:hir.rs-4062 */ 
/* FP:hir.rs-4063 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-4064 */ pub struct FieldDef<'hir> {
/* FP:hir.rs-4065 */     pub span: Span,
/* FP:hir.rs-4066 */     pub vis_span: Span,
/* FP:hir.rs-4067 */     pub ident: Ident,
/* FP:hir.rs-4068 */     #[stable_hasher(ignore)]
/* FP:hir.rs-4069 */     pub hir_id: HirId,
/* FP:hir.rs-4070 */     pub def_id: LocalDefId,
/* FP:hir.rs-4071 */     pub ty: &'hir Ty<'hir>,
/* FP:hir.rs-4072 */     pub safety: Safety,
/* FP:hir.rs-4073 */     pub default: Option<&'hir AnonConst>,
/* FP:hir.rs-4074 */ }
/* FP:hir.rs-4075 */ 
/* FP:hir.rs-4076 */ impl FieldDef<'_> {
/* FP:hir.rs-4077 */     // Still necessary in couple of places
/* FP:hir.rs-4078 */     pub fn is_positional(&self) -> bool {
/* FP:hir.rs-4079 */         self.ident.as_str().as_bytes()[0].is_ascii_digit()
/* FP:hir.rs-4080 */     }
/* FP:hir.rs-4081 */ }
/* FP:hir.rs-4082 */ 
/* FP:hir.rs-4083 */ /// Fields and constructor IDs of enum variants and structs.
/* FP:hir.rs-4084 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-4085 */ pub enum VariantData<'hir> {
/* FP:hir.rs-4086 */     /// A struct variant.
/* FP:hir.rs-4087 */     ///
/* FP:hir.rs-4088 */     /// E.g., `Bar { .. }` as in `enum Foo { Bar { .. } }`.
/* FP:hir.rs-4089 */     Struct { fields: &'hir [FieldDef<'hir>], recovered: ast::Recovered },
/* FP:hir.rs-4090 */     /// A tuple variant.
/* FP:hir.rs-4091 */     ///
/* FP:hir.rs-4092 */     /// E.g., `Bar(..)` as in `enum Foo { Bar(..) }`.
/* FP:hir.rs-4093 */     Tuple(&'hir [FieldDef<'hir>], #[stable_hasher(ignore)] HirId, LocalDefId),
/* FP:hir.rs-4094 */     /// A unit variant.
/* FP:hir.rs-4095 */     ///
/* FP:hir.rs-4096 */     /// E.g., `Bar = ..` as in `enum Foo { Bar = .. }`.
/* FP:hir.rs-4097 */     Unit(#[stable_hasher(ignore)] HirId, LocalDefId),
/* FP:hir.rs-4098 */ }
/* FP:hir.rs-4099 */ 
/* FP:hir.rs-4100 */ impl<'hir> VariantData<'hir> {
/* FP:hir.rs-4101 */     /// Return the fields of this variant.
/* FP:hir.rs-4102 */     pub fn fields(&self) -> &'hir [FieldDef<'hir>] {
/* FP:hir.rs-4103 */         match *self {
/* FP:hir.rs-4104 */             VariantData::Struct { fields, .. } | VariantData::Tuple(fields, ..) => fields,
/* FP:hir.rs-4105 */             _ => &[],
/* FP:hir.rs-4106 */         }
/* FP:hir.rs-4107 */     }
/* FP:hir.rs-4108 */ 
/* FP:hir.rs-4109 */     pub fn ctor(&self) -> Option<(CtorKind, HirId, LocalDefId)> {
/* FP:hir.rs-4110 */         match *self {
/* FP:hir.rs-4111 */             VariantData::Tuple(_, hir_id, def_id) => Some((CtorKind::Fn, hir_id, def_id)),
/* FP:hir.rs-4112 */             VariantData::Unit(hir_id, def_id) => Some((CtorKind::Const, hir_id, def_id)),
/* FP:hir.rs-4113 */             VariantData::Struct { .. } => None,
/* FP:hir.rs-4114 */         }
/* FP:hir.rs-4115 */     }
/* FP:hir.rs-4116 */ 
/* FP:hir.rs-4117 */     #[inline]
/* FP:hir.rs-4118 */     pub fn ctor_kind(&self) -> Option<CtorKind> {
/* FP:hir.rs-4119 */         self.ctor().map(|(kind, ..)| kind)
/* FP:hir.rs-4120 */     }
/* FP:hir.rs-4121 */ 
/* FP:hir.rs-4122 */     /// Return the `HirId` of this variant's constructor, if it has one.
/* FP:hir.rs-4123 */     #[inline]
/* FP:hir.rs-4124 */     pub fn ctor_hir_id(&self) -> Option<HirId> {
/* FP:hir.rs-4125 */         self.ctor().map(|(_, hir_id, _)| hir_id)
/* FP:hir.rs-4126 */     }
/* FP:hir.rs-4127 */ 
/* FP:hir.rs-4128 */     /// Return the `LocalDefId` of this variant's constructor, if it has one.
/* FP:hir.rs-4129 */     #[inline]
/* FP:hir.rs-4130 */     pub fn ctor_def_id(&self) -> Option<LocalDefId> {
/* FP:hir.rs-4131 */         self.ctor().map(|(.., def_id)| def_id)
/* FP:hir.rs-4132 */     }
/* FP:hir.rs-4133 */ }
/* FP:hir.rs-4134 */ 
/* FP:hir.rs-4135 */ // The bodies for items are stored "out of line", in a separate
/* FP:hir.rs-4136 */ // hashmap in the `Crate`. Here we just record the hir-id of the item
/* FP:hir.rs-4137 */ // so it can fetched later.
/* FP:hir.rs-4138 */ #[derive(Copy, Clone, PartialEq, Eq, Encodable, Decodable, Debug, Hash, HashStable_Generic)]
/* FP:hir.rs-4139 */ pub struct ItemId {
/* FP:hir.rs-4140 */     pub owner_id: OwnerId,
/* FP:hir.rs-4141 */ }
/* FP:hir.rs-4142 */ 
/* FP:hir.rs-4143 */ impl ItemId {
/* FP:hir.rs-4144 */     #[inline]
/* FP:hir.rs-4145 */     pub fn hir_id(&self) -> HirId {
/* FP:hir.rs-4146 */         // Items are always HIR owners.
/* FP:hir.rs-4147 */         HirId::make_owner(self.owner_id.def_id)
/* FP:hir.rs-4148 */     }
/* FP:hir.rs-4149 */ }
/* FP:hir.rs-4150 */ 
/* FP:hir.rs-4151 */ /// An item
/* FP:hir.rs-4152 */ ///
/* FP:hir.rs-4153 */ /// For more details, see the [rust lang reference].
/* FP:hir.rs-4154 */ /// Note that the reference does not document nightly-only features.
/* FP:hir.rs-4155 */ /// There may be also slight differences in the names and representation of AST nodes between
/* FP:hir.rs-4156 */ /// the compiler and the reference.
/* FP:hir.rs-4157 */ ///
/* FP:hir.rs-4158 */ /// [rust lang reference]: https://doc.rust-lang.org/reference/items.html
/* FP:hir.rs-4159 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-4160 */ pub struct Item<'hir> {
/* FP:hir.rs-4161 */     pub owner_id: OwnerId,
/* FP:hir.rs-4162 */     pub kind: ItemKind<'hir>,
/* FP:hir.rs-4163 */     pub span: Span,
/* FP:hir.rs-4164 */     pub vis_span: Span,
/* FP:hir.rs-4165 */     pub has_delayed_lints: bool,
/* FP:hir.rs-4166 */ }
/* FP:hir.rs-4167 */ 
/* FP:hir.rs-4168 */ impl<'hir> Item<'hir> {
/* FP:hir.rs-4169 */     #[inline]
/* FP:hir.rs-4170 */     pub fn hir_id(&self) -> HirId {
/* FP:hir.rs-4171 */         // Items are always HIR owners.
/* FP:hir.rs-4172 */         HirId::make_owner(self.owner_id.def_id)
/* FP:hir.rs-4173 */     }
/* FP:hir.rs-4174 */ 
/* FP:hir.rs-4175 */     pub fn item_id(&self) -> ItemId {
/* FP:hir.rs-4176 */         ItemId { owner_id: self.owner_id }
/* FP:hir.rs-4177 */     }
/* FP:hir.rs-4178 */ 
/* FP:hir.rs-4179 */     /// Check if this is an [`ItemKind::Enum`], [`ItemKind::Struct`] or
/* FP:hir.rs-4180 */     /// [`ItemKind::Union`].
/* FP:hir.rs-4181 */     pub fn is_adt(&self) -> bool {
/* FP:hir.rs-4182 */         matches!(self.kind, ItemKind::Enum(..) | ItemKind::Struct(..) | ItemKind::Union(..))
/* FP:hir.rs-4183 */     }
/* FP:hir.rs-4184 */ 
/* FP:hir.rs-4185 */     /// Check if this is an [`ItemKind::Struct`] or [`ItemKind::Union`].
/* FP:hir.rs-4186 */     pub fn is_struct_or_union(&self) -> bool {
/* FP:hir.rs-4187 */         matches!(self.kind, ItemKind::Struct(..) | ItemKind::Union(..))
/* FP:hir.rs-4188 */     }
/* FP:hir.rs-4189 */ 
/* FP:hir.rs-4190 */     expect_methods_self_kind! {
/* FP:hir.rs-4191 */         expect_extern_crate, (Option<Symbol>, Ident),
/* FP:hir.rs-4192 */             ItemKind::ExternCrate(s, ident), (*s, *ident);
/* FP:hir.rs-4193 */ 
/* FP:hir.rs-4194 */         expect_use, (&'hir UsePath<'hir>, UseKind), ItemKind::Use(p, uk), (p, *uk);
/* FP:hir.rs-4195 */ 
/* FP:hir.rs-4196 */         expect_static, (Mutability, Ident, &'hir Ty<'hir>, BodyId),
/* FP:hir.rs-4197 */             ItemKind::Static(mutbl, ident, ty, body), (*mutbl, *ident, ty, *body);
/* FP:hir.rs-4198 */ 
/* FP:hir.rs-4199 */         expect_const, (Ident, &'hir Generics<'hir>, &'hir Ty<'hir>, BodyId),
/* FP:hir.rs-4200 */             ItemKind::Const(ident, generics, ty, body), (*ident, generics, ty, *body);
/* FP:hir.rs-4201 */ 
/* FP:hir.rs-4202 */         expect_fn, (Ident, &FnSig<'hir>, &'hir Generics<'hir>, BodyId),
/* FP:hir.rs-4203 */             ItemKind::Fn { ident, sig, generics, body, .. }, (*ident, sig, generics, *body);
/* FP:hir.rs-4204 */ 
/* FP:hir.rs-4205 */         expect_macro, (Ident, &ast::MacroDef, MacroKinds),
/* FP:hir.rs-4206 */             ItemKind::Macro(ident, def, mk), (*ident, def, *mk);
/* FP:hir.rs-4207 */ 
/* FP:hir.rs-4208 */         expect_mod, (Ident, &'hir Mod<'hir>), ItemKind::Mod(ident, m), (*ident, m);
/* FP:hir.rs-4209 */ 
/* FP:hir.rs-4210 */         expect_foreign_mod, (ExternAbi, &'hir [ForeignItemId]),
/* FP:hir.rs-4211 */             ItemKind::ForeignMod { abi, items }, (*abi, items);
/* FP:hir.rs-4212 */ 
/* FP:hir.rs-4213 */         expect_global_asm, &'hir InlineAsm<'hir>, ItemKind::GlobalAsm { asm, .. }, asm;
/* FP:hir.rs-4214 */ 
/* FP:hir.rs-4215 */         expect_ty_alias, (Ident, &'hir Generics<'hir>, &'hir Ty<'hir>),
/* FP:hir.rs-4216 */             ItemKind::TyAlias(ident, generics, ty), (*ident, generics, ty);
/* FP:hir.rs-4217 */ 
/* FP:hir.rs-4218 */         expect_enum, (Ident, &'hir Generics<'hir>, &EnumDef<'hir>),
/* FP:hir.rs-4219 */             ItemKind::Enum(ident, generics, def), (*ident, generics, def);
/* FP:hir.rs-4220 */ 
/* FP:hir.rs-4221 */         expect_struct, (Ident, &'hir Generics<'hir>, &VariantData<'hir>),
/* FP:hir.rs-4222 */             ItemKind::Struct(ident, generics, data), (*ident, generics, data);
/* FP:hir.rs-4223 */ 
/* FP:hir.rs-4224 */         expect_union, (Ident, &'hir Generics<'hir>, &VariantData<'hir>),
/* FP:hir.rs-4225 */             ItemKind::Union(ident, generics, data), (*ident, generics, data);
/* FP:hir.rs-4226 */ 
/* FP:hir.rs-4227 */         expect_trait,
/* FP:hir.rs-4228 */             (
/* FP:hir.rs-4229 */                 Constness,
/* FP:hir.rs-4230 */                 IsAuto,
/* FP:hir.rs-4231 */                 Safety,
/* FP:hir.rs-4232 */                 Ident,
/* FP:hir.rs-4233 */                 &'hir Generics<'hir>,
/* FP:hir.rs-4234 */                 GenericBounds<'hir>,
/* FP:hir.rs-4235 */                 &'hir [TraitItemId]
/* FP:hir.rs-4236 */             ),
/* FP:hir.rs-4237 */             ItemKind::Trait(constness, is_auto, safety, ident, generics, bounds, items),
/* FP:hir.rs-4238 */             (*constness, *is_auto, *safety, *ident, generics, bounds, items);
/* FP:hir.rs-4239 */ 
/* FP:hir.rs-4240 */         expect_trait_alias, (Ident, &'hir Generics<'hir>, GenericBounds<'hir>),
/* FP:hir.rs-4241 */             ItemKind::TraitAlias(ident, generics, bounds), (*ident, generics, bounds);
/* FP:hir.rs-4242 */ 
/* FP:hir.rs-4243 */         expect_impl, &Impl<'hir>, ItemKind::Impl(imp), imp;
/* FP:hir.rs-4244 */     }
/* FP:hir.rs-4245 */ }
/* FP:hir.rs-4246 */ 
/* FP:hir.rs-4247 */ #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
/* FP:hir.rs-4248 */ #[derive(Encodable, Decodable, HashStable_Generic)]
/* FP:hir.rs-4249 */ pub enum Safety {
/* FP:hir.rs-4250 */     Unsafe,
/* FP:hir.rs-4251 */     Safe,
/* FP:hir.rs-4252 */ }
/* FP:hir.rs-4253 */ 
/* FP:hir.rs-4254 */ impl Safety {
/* FP:hir.rs-4255 */     pub fn prefix_str(self) -> &'static str {
/* FP:hir.rs-4256 */         match self {
/* FP:hir.rs-4257 */             Self::Unsafe => "unsafe ",
/* FP:hir.rs-4258 */             Self::Safe => "",
/* FP:hir.rs-4259 */         }
/* FP:hir.rs-4260 */     }
/* FP:hir.rs-4261 */ 
/* FP:hir.rs-4262 */     #[inline]
/* FP:hir.rs-4263 */     pub fn is_unsafe(self) -> bool {
/* FP:hir.rs-4264 */         !self.is_safe()
/* FP:hir.rs-4265 */     }
/* FP:hir.rs-4266 */ 
/* FP:hir.rs-4267 */     #[inline]
/* FP:hir.rs-4268 */     pub fn is_safe(self) -> bool {
/* FP:hir.rs-4269 */         match self {
/* FP:hir.rs-4270 */             Self::Unsafe => false,
/* FP:hir.rs-4271 */             Self::Safe => true,
/* FP:hir.rs-4272 */         }
/* FP:hir.rs-4273 */     }
/* FP:hir.rs-4274 */ }
/* FP:hir.rs-4275 */ 
/* FP:hir.rs-4276 */ impl fmt::Display for Safety {
/* FP:hir.rs-4277 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-4278 */         f.write_str(match *self {
/* FP:hir.rs-4279 */             Self::Unsafe => "unsafe",
/* FP:hir.rs-4280 */             Self::Safe => "safe",
/* FP:hir.rs-4281 */         })
/* FP:hir.rs-4282 */     }
/* FP:hir.rs-4283 */ }
/* FP:hir.rs-4284 */ 
/* FP:hir.rs-4285 */ #[derive(Copy, Clone, PartialEq, Eq, Debug, Encodable, Decodable, HashStable_Generic)]
/* FP:hir.rs-4286 */ pub enum Constness {
/* FP:hir.rs-4287 */     Const,
/* FP:hir.rs-4288 */     NotConst,
/* FP:hir.rs-4289 */ }
/* FP:hir.rs-4290 */ 
/* FP:hir.rs-4291 */ impl fmt::Display for Constness {
/* FP:hir.rs-4292 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:hir.rs-4293 */         f.write_str(match *self {
/* FP:hir.rs-4294 */             Self::Const => "const",
/* FP:hir.rs-4295 */             Self::NotConst => "non-const",
/* FP:hir.rs-4296 */         })
/* FP:hir.rs-4297 */     }
/* FP:hir.rs-4298 */ }
/* FP:hir.rs-4299 */ 
/* FP:hir.rs-4300 */ /// The actual safety specified in syntax. We may treat
/* FP:hir.rs-4301 */ /// its safety different within the type system to create a
/* FP:hir.rs-4302 */ /// "sound by default" system that needs checking this enum
/* FP:hir.rs-4303 */ /// explicitly to allow unsafe operations.
/* FP:hir.rs-4304 */ #[derive(Copy, Clone, Debug, HashStable_Generic, PartialEq, Eq)]
/* FP:hir.rs-4305 */ pub enum HeaderSafety {
/* FP:hir.rs-4306 */     /// A safe function annotated with `#[target_features]`.
/* FP:hir.rs-4307 */     /// The type system treats this function as an unsafe function,
/* FP:hir.rs-4308 */     /// but safety checking will check this enum to treat it as safe
/* FP:hir.rs-4309 */     /// and allowing calling other safe target feature functions with
/* FP:hir.rs-4310 */     /// the same features without requiring an additional unsafe block.
/* FP:hir.rs-4311 */     SafeTargetFeatures,
/* FP:hir.rs-4312 */     Normal(Safety),
/* FP:hir.rs-4313 */ }
/* FP:hir.rs-4314 */ 
/* FP:hir.rs-4315 */ impl From<Safety> for HeaderSafety {
/* FP:hir.rs-4316 */     fn from(v: Safety) -> Self {
/* FP:hir.rs-4317 */         Self::Normal(v)
/* FP:hir.rs-4318 */     }
/* FP:hir.rs-4319 */ }
/* FP:hir.rs-4320 */ 
/* FP:hir.rs-4321 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-4322 */ pub struct FnHeader {
/* FP:hir.rs-4323 */     pub safety: HeaderSafety,
/* FP:hir.rs-4324 */     pub constness: Constness,
/* FP:hir.rs-4325 */     pub asyncness: IsAsync,
/* FP:hir.rs-4326 */     pub abi: ExternAbi,
/* FP:hir.rs-4327 */ }
/* FP:hir.rs-4328 */ 
/* FP:hir.rs-4329 */ impl FnHeader {
/* FP:hir.rs-4330 */     pub fn is_async(&self) -> bool {
/* FP:hir.rs-4331 */         matches!(self.asyncness, IsAsync::Async(_))
/* FP:hir.rs-4332 */     }
/* FP:hir.rs-4333 */ 
/* FP:hir.rs-4334 */     pub fn is_const(&self) -> bool {
/* FP:hir.rs-4335 */         matches!(self.constness, Constness::Const)
/* FP:hir.rs-4336 */     }
/* FP:hir.rs-4337 */ 
/* FP:hir.rs-4338 */     pub fn is_unsafe(&self) -> bool {
/* FP:hir.rs-4339 */         self.safety().is_unsafe()
/* FP:hir.rs-4340 */     }
/* FP:hir.rs-4341 */ 
/* FP:hir.rs-4342 */     pub fn is_safe(&self) -> bool {
/* FP:hir.rs-4343 */         self.safety().is_safe()
/* FP:hir.rs-4344 */     }
/* FP:hir.rs-4345 */ 
/* FP:hir.rs-4346 */     pub fn safety(&self) -> Safety {
/* FP:hir.rs-4347 */         match self.safety {
/* FP:hir.rs-4348 */             HeaderSafety::SafeTargetFeatures => Safety::Unsafe,
/* FP:hir.rs-4349 */             HeaderSafety::Normal(safety) => safety,
/* FP:hir.rs-4350 */         }
/* FP:hir.rs-4351 */     }
/* FP:hir.rs-4352 */ }
/* FP:hir.rs-4353 */ 
/* FP:hir.rs-4354 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-4355 */ pub enum ItemKind<'hir> {
/* FP:hir.rs-4356 */     /// An `extern crate` item, with optional *original* crate name if the crate was renamed.
/* FP:hir.rs-4357 */     ///
/* FP:hir.rs-4358 */     /// E.g., `extern crate foo` or `extern crate foo_bar as foo`.
/* FP:hir.rs-4359 */     ExternCrate(Option<Symbol>, Ident),
/* FP:hir.rs-4360 */ 
/* FP:hir.rs-4361 */     /// `use foo::bar::*;` or `use foo::bar::baz as quux;`
/* FP:hir.rs-4362 */     ///
/* FP:hir.rs-4363 */     /// or just
/* FP:hir.rs-4364 */     ///
/* FP:hir.rs-4365 */     /// `use foo::bar::baz;` (with `as baz` implicitly on the right).
/* FP:hir.rs-4366 */     Use(&'hir UsePath<'hir>, UseKind),
/* FP:hir.rs-4367 */ 
/* FP:hir.rs-4368 */     /// A `static` item.
/* FP:hir.rs-4369 */     Static(Mutability, Ident, &'hir Ty<'hir>, BodyId),
/* FP:hir.rs-4370 */     /// A `const` item.
/* FP:hir.rs-4371 */     Const(Ident, &'hir Generics<'hir>, &'hir Ty<'hir>, BodyId),
/* FP:hir.rs-4372 */     /// A function declaration.
/* FP:hir.rs-4373 */     Fn {
/* FP:hir.rs-4374 */         sig: FnSig<'hir>,
/* FP:hir.rs-4375 */         ident: Ident,
/* FP:hir.rs-4376 */         generics: &'hir Generics<'hir>,
/* FP:hir.rs-4377 */         body: BodyId,
/* FP:hir.rs-4378 */         /// Whether this function actually has a body.
/* FP:hir.rs-4379 */         /// For functions without a body, `body` is synthesized (to avoid ICEs all over the
/* FP:hir.rs-4380 */         /// compiler), but that code should never be translated.
/* FP:hir.rs-4381 */         has_body: bool,
/* FP:hir.rs-4382 */     },
/* FP:hir.rs-4383 */     /// A MBE macro definition (`macro_rules!` or `macro`).
/* FP:hir.rs-4384 */     Macro(Ident, &'hir ast::MacroDef, MacroKinds),
/* FP:hir.rs-4385 */     /// A module.
/* FP:hir.rs-4386 */     Mod(Ident, &'hir Mod<'hir>),
/* FP:hir.rs-4387 */     /// An external module, e.g. `unsafe extern { .. }`.
/* FP:hir.rs-4388 */     ForeignMod { abi: ExternAbi, items: &'hir [ForeignItemId] },
/* FP:hir.rs-4389 */     /// Module-level inline assembly (from `global_asm!`).
/* FP:hir.rs-4390 */     GlobalAsm {
/* FP:hir.rs-4391 */         asm: &'hir InlineAsm<'hir>,
/* FP:hir.rs-4392 */         /// A fake body which stores typeck results for the global asm's sym_fn
/* FP:hir.rs-4393 */         /// operands, which are represented as path expressions. This body contains
/* FP:hir.rs-4394 */         /// a single [`ExprKind::InlineAsm`] which points to the asm in the field
/* FP:hir.rs-4395 */         /// above, and which is typechecked like a inline asm expr just for the
/* FP:hir.rs-4396 */         /// typeck results.
/* FP:hir.rs-4397 */         fake_body: BodyId,
/* FP:hir.rs-4398 */     },
/* FP:hir.rs-4399 */     /// A type alias, e.g., `type Foo = Bar<u8>`.
/* FP:hir.rs-4400 */     TyAlias(Ident, &'hir Generics<'hir>, &'hir Ty<'hir>),
/* FP:hir.rs-4401 */     /// An enum definition, e.g., `enum Foo<A, B> { C<A>, D<B> }`.
/* FP:hir.rs-4402 */     Enum(Ident, &'hir Generics<'hir>, EnumDef<'hir>),
/* FP:hir.rs-4403 */     /// A struct definition, e.g., `struct Foo<A> {x: A}`.
/* FP:hir.rs-4404 */     Struct(Ident, &'hir Generics<'hir>, VariantData<'hir>),
/* FP:hir.rs-4405 */     /// A union definition, e.g., `union Foo<A, B> {x: A, y: B}`.
/* FP:hir.rs-4406 */     Union(Ident, &'hir Generics<'hir>, VariantData<'hir>),
/* FP:hir.rs-4407 */     /// A trait definition.
/* FP:hir.rs-4408 */     Trait(
/* FP:hir.rs-4409 */         Constness,
/* FP:hir.rs-4410 */         IsAuto,
/* FP:hir.rs-4411 */         Safety,
/* FP:hir.rs-4412 */         Ident,
/* FP:hir.rs-4413 */         &'hir Generics<'hir>,
/* FP:hir.rs-4414 */         GenericBounds<'hir>,
/* FP:hir.rs-4415 */         &'hir [TraitItemId],
/* FP:hir.rs-4416 */     ),
/* FP:hir.rs-4417 */     /// A trait alias.
/* FP:hir.rs-4418 */     TraitAlias(Ident, &'hir Generics<'hir>, GenericBounds<'hir>),
/* FP:hir.rs-4419 */ 
/* FP:hir.rs-4420 */     /// An implementation, e.g., `impl<A> Trait for Foo { .. }`.
/* FP:hir.rs-4421 */     Impl(Impl<'hir>),
/* FP:hir.rs-4422 */ }
/* FP:hir.rs-4423 */ 
/* FP:hir.rs-4424 */ /// Represents an impl block declaration.
/* FP:hir.rs-4425 */ ///
/* FP:hir.rs-4426 */ /// E.g., `impl $Type { .. }` or `impl $Trait for $Type { .. }`
/* FP:hir.rs-4427 */ /// Refer to [`ImplItem`] for an associated item within an impl block.
/* FP:hir.rs-4428 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-4429 */ pub struct Impl<'hir> {
/* FP:hir.rs-4430 */     pub generics: &'hir Generics<'hir>,
/* FP:hir.rs-4431 */     pub of_trait: Option<&'hir TraitImplHeader<'hir>>,
/* FP:hir.rs-4432 */     pub self_ty: &'hir Ty<'hir>,
/* FP:hir.rs-4433 */     pub items: &'hir [ImplItemId],
/* FP:hir.rs-4434 */ }
/* FP:hir.rs-4435 */ 
/* FP:hir.rs-4436 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-4437 */ pub struct TraitImplHeader<'hir> {
/* FP:hir.rs-4438 */     pub constness: Constness,
/* FP:hir.rs-4439 */     pub safety: Safety,
/* FP:hir.rs-4440 */     pub polarity: ImplPolarity,
/* FP:hir.rs-4441 */     pub defaultness: Defaultness,
/* FP:hir.rs-4442 */     // We do not put a `Span` in `Defaultness` because it breaks foreign crate metadata
/* FP:hir.rs-4443 */     // decoding as `Span`s cannot be decoded when a `Session` is not available.
/* FP:hir.rs-4444 */     pub defaultness_span: Option<Span>,
/* FP:hir.rs-4445 */     pub trait_ref: TraitRef<'hir>,
/* FP:hir.rs-4446 */ }
/* FP:hir.rs-4447 */ 
/* FP:hir.rs-4448 */ impl ItemKind<'_> {
/* FP:hir.rs-4449 */     pub fn ident(&self) -> Option<Ident> {
/* FP:hir.rs-4450 */         match *self {
/* FP:hir.rs-4451 */             ItemKind::ExternCrate(_, ident)
/* FP:hir.rs-4452 */             | ItemKind::Use(_, UseKind::Single(ident))
/* FP:hir.rs-4453 */             | ItemKind::Static(_, ident, ..)
/* FP:hir.rs-4454 */             | ItemKind::Const(ident, ..)
/* FP:hir.rs-4455 */             | ItemKind::Fn { ident, .. }
/* FP:hir.rs-4456 */             | ItemKind::Macro(ident, ..)
/* FP:hir.rs-4457 */             | ItemKind::Mod(ident, ..)
/* FP:hir.rs-4458 */             | ItemKind::TyAlias(ident, ..)
/* FP:hir.rs-4459 */             | ItemKind::Enum(ident, ..)
/* FP:hir.rs-4460 */             | ItemKind::Struct(ident, ..)
/* FP:hir.rs-4461 */             | ItemKind::Union(ident, ..)
/* FP:hir.rs-4462 */             | ItemKind::Trait(_, _, _, ident, ..)
/* FP:hir.rs-4463 */             | ItemKind::TraitAlias(ident, ..) => Some(ident),
/* FP:hir.rs-4464 */ 
/* FP:hir.rs-4465 */             ItemKind::Use(_, UseKind::Glob | UseKind::ListStem)
/* FP:hir.rs-4466 */             | ItemKind::ForeignMod { .. }
/* FP:hir.rs-4467 */             | ItemKind::GlobalAsm { .. }
/* FP:hir.rs-4468 */             | ItemKind::Impl(_) => None,
/* FP:hir.rs-4469 */         }
/* FP:hir.rs-4470 */     }
/* FP:hir.rs-4471 */ 
/* FP:hir.rs-4472 */     pub fn generics(&self) -> Option<&Generics<'_>> {
/* FP:hir.rs-4473 */         Some(match self {
/* FP:hir.rs-4474 */             ItemKind::Fn { generics, .. }
/* FP:hir.rs-4475 */             | ItemKind::TyAlias(_, generics, _)
/* FP:hir.rs-4476 */             | ItemKind::Const(_, generics, _, _)
/* FP:hir.rs-4477 */             | ItemKind::Enum(_, generics, _)
/* FP:hir.rs-4478 */             | ItemKind::Struct(_, generics, _)
/* FP:hir.rs-4479 */             | ItemKind::Union(_, generics, _)
/* FP:hir.rs-4480 */             | ItemKind::Trait(_, _, _, _, generics, _, _)
/* FP:hir.rs-4481 */             | ItemKind::TraitAlias(_, generics, _)
/* FP:hir.rs-4482 */             | ItemKind::Impl(Impl { generics, .. }) => generics,
/* FP:hir.rs-4483 */             _ => return None,
/* FP:hir.rs-4484 */         })
/* FP:hir.rs-4485 */     }
/* FP:hir.rs-4486 */ }
/* FP:hir.rs-4487 */ 
/* FP:hir.rs-4488 */ // The bodies for items are stored "out of line", in a separate
/* FP:hir.rs-4489 */ // hashmap in the `Crate`. Here we just record the hir-id of the item
/* FP:hir.rs-4490 */ // so it can fetched later.
/* FP:hir.rs-4491 */ #[derive(Copy, Clone, PartialEq, Eq, Encodable, Decodable, Debug, HashStable_Generic)]
/* FP:hir.rs-4492 */ pub struct ForeignItemId {
/* FP:hir.rs-4493 */     pub owner_id: OwnerId,
/* FP:hir.rs-4494 */ }
/* FP:hir.rs-4495 */ 
/* FP:hir.rs-4496 */ impl ForeignItemId {
/* FP:hir.rs-4497 */     #[inline]
/* FP:hir.rs-4498 */     pub fn hir_id(&self) -> HirId {
/* FP:hir.rs-4499 */         // Items are always HIR owners.
/* FP:hir.rs-4500 */         HirId::make_owner(self.owner_id.def_id)
/* FP:hir.rs-4501 */     }
/* FP:hir.rs-4502 */ }
/* FP:hir.rs-4503 */ 
/* FP:hir.rs-4504 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-4505 */ pub struct ForeignItem<'hir> {
/* FP:hir.rs-4506 */     pub ident: Ident,
/* FP:hir.rs-4507 */     pub kind: ForeignItemKind<'hir>,
/* FP:hir.rs-4508 */     pub owner_id: OwnerId,
/* FP:hir.rs-4509 */     pub span: Span,
/* FP:hir.rs-4510 */     pub vis_span: Span,
/* FP:hir.rs-4511 */     pub has_delayed_lints: bool,
/* FP:hir.rs-4512 */ }
/* FP:hir.rs-4513 */ 
/* FP:hir.rs-4514 */ impl ForeignItem<'_> {
/* FP:hir.rs-4515 */     #[inline]
/* FP:hir.rs-4516 */     pub fn hir_id(&self) -> HirId {
/* FP:hir.rs-4517 */         // Items are always HIR owners.
/* FP:hir.rs-4518 */         HirId::make_owner(self.owner_id.def_id)
/* FP:hir.rs-4519 */     }
/* FP:hir.rs-4520 */ 
/* FP:hir.rs-4521 */     pub fn foreign_item_id(&self) -> ForeignItemId {
/* FP:hir.rs-4522 */         ForeignItemId { owner_id: self.owner_id }
/* FP:hir.rs-4523 */     }
/* FP:hir.rs-4524 */ }
/* FP:hir.rs-4525 */ 
/* FP:hir.rs-4526 */ /// An item within an `extern` block.
/* FP:hir.rs-4527 */ #[derive(Debug, Clone, Copy, HashStable_Generic)]
/* FP:hir.rs-4528 */ pub enum ForeignItemKind<'hir> {
/* FP:hir.rs-4529 */     /// A foreign function.
/* FP:hir.rs-4530 */     ///
/* FP:hir.rs-4531 */     /// All argument idents are actually always present (i.e. `Some`), but
/* FP:hir.rs-4532 */     /// `&[Option<Ident>]` is used because of code paths shared with `TraitFn`
/* FP:hir.rs-4533 */     /// and `FnPtrTy`. The sharing is due to all of these cases not allowing
/* FP:hir.rs-4534 */     /// arbitrary patterns for parameters.
/* FP:hir.rs-4535 */     Fn(FnSig<'hir>, &'hir [Option<Ident>], &'hir Generics<'hir>),
/* FP:hir.rs-4536 */     /// A foreign static item (`static ext: u8`).
/* FP:hir.rs-4537 */     Static(&'hir Ty<'hir>, Mutability, Safety),
/* FP:hir.rs-4538 */     /// A foreign type.
/* FP:hir.rs-4539 */     Type,
/* FP:hir.rs-4540 */ }
/* FP:hir.rs-4541 */ 
/* FP:hir.rs-4542 */ /// A variable captured by a closure.
/* FP:hir.rs-4543 */ #[derive(Debug, Copy, Clone, HashStable_Generic)]
/* FP:hir.rs-4544 */ pub struct Upvar {
/* FP:hir.rs-4545 */     /// First span where it is accessed (there can be multiple).
/* FP:hir.rs-4546 */     pub span: Span,
/* FP:hir.rs-4547 */ }
/* FP:hir.rs-4548 */ 
/* FP:hir.rs-4549 */ // The TraitCandidate's import_ids is empty if the trait is defined in the same module, and
/* FP:hir.rs-4550 */ // has length > 0 if the trait is found through an chain of imports, starting with the
/* FP:hir.rs-4551 */ // import/use statement in the scope where the trait is used.
/* FP:hir.rs-4552 */ #[derive(Debug, Clone, HashStable_Generic)]
/* FP:hir.rs-4553 */ pub struct TraitCandidate {
/* FP:hir.rs-4554 */     pub def_id: DefId,
/* FP:hir.rs-4555 */     pub import_ids: SmallVec<[LocalDefId; 1]>,
/* FP:hir.rs-4556 */ }
/* FP:hir.rs-4557 */ 
/* FP:hir.rs-4558 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-4559 */ pub enum OwnerNode<'hir> {
/* FP:hir.rs-4560 */     Item(&'hir Item<'hir>),
/* FP:hir.rs-4561 */     ForeignItem(&'hir ForeignItem<'hir>),
/* FP:hir.rs-4562 */     TraitItem(&'hir TraitItem<'hir>),
/* FP:hir.rs-4563 */     ImplItem(&'hir ImplItem<'hir>),
/* FP:hir.rs-4564 */     Crate(&'hir Mod<'hir>),
/* FP:hir.rs-4565 */     Synthetic,
/* FP:hir.rs-4566 */ }
/* FP:hir.rs-4567 */ 
/* FP:hir.rs-4568 */ impl<'hir> OwnerNode<'hir> {
/* FP:hir.rs-4569 */     pub fn span(&self) -> Span {
/* FP:hir.rs-4570 */         match self {
/* FP:hir.rs-4571 */             OwnerNode::Item(Item { span, .. })
/* FP:hir.rs-4572 */             | OwnerNode::ForeignItem(ForeignItem { span, .. })
/* FP:hir.rs-4573 */             | OwnerNode::ImplItem(ImplItem { span, .. })
/* FP:hir.rs-4574 */             | OwnerNode::TraitItem(TraitItem { span, .. }) => *span,
/* FP:hir.rs-4575 */             OwnerNode::Crate(Mod { spans: ModSpans { inner_span, .. }, .. }) => *inner_span,
/* FP:hir.rs-4576 */             OwnerNode::Synthetic => unreachable!(),
/* FP:hir.rs-4577 */         }
/* FP:hir.rs-4578 */     }
/* FP:hir.rs-4579 */ 
/* FP:hir.rs-4580 */     pub fn fn_sig(self) -> Option<&'hir FnSig<'hir>> {
/* FP:hir.rs-4581 */         match self {
/* FP:hir.rs-4582 */             OwnerNode::TraitItem(TraitItem { kind: TraitItemKind::Fn(fn_sig, _), .. })
/* FP:hir.rs-4583 */             | OwnerNode::ImplItem(ImplItem { kind: ImplItemKind::Fn(fn_sig, _), .. })
/* FP:hir.rs-4584 */             | OwnerNode::Item(Item { kind: ItemKind::Fn { sig: fn_sig, .. }, .. })
/* FP:hir.rs-4585 */             | OwnerNode::ForeignItem(ForeignItem {
/* FP:hir.rs-4586 */                 kind: ForeignItemKind::Fn(fn_sig, _, _), ..
/* FP:hir.rs-4587 */             }) => Some(fn_sig),
/* FP:hir.rs-4588 */             _ => None,
/* FP:hir.rs-4589 */         }
/* FP:hir.rs-4590 */     }
/* FP:hir.rs-4591 */ 
/* FP:hir.rs-4592 */     pub fn fn_decl(self) -> Option<&'hir FnDecl<'hir>> {
/* FP:hir.rs-4593 */         match self {
/* FP:hir.rs-4594 */             OwnerNode::TraitItem(TraitItem { kind: TraitItemKind::Fn(fn_sig, _), .. })
/* FP:hir.rs-4595 */             | OwnerNode::ImplItem(ImplItem { kind: ImplItemKind::Fn(fn_sig, _), .. })
/* FP:hir.rs-4596 */             | OwnerNode::Item(Item { kind: ItemKind::Fn { sig: fn_sig, .. }, .. })
/* FP:hir.rs-4597 */             | OwnerNode::ForeignItem(ForeignItem {
/* FP:hir.rs-4598 */                 kind: ForeignItemKind::Fn(fn_sig, _, _), ..
/* FP:hir.rs-4599 */             }) => Some(fn_sig.decl),
/* FP:hir.rs-4600 */             _ => None,
/* FP:hir.rs-4601 */         }
/* FP:hir.rs-4602 */     }
/* FP:hir.rs-4603 */ 
/* FP:hir.rs-4604 */     pub fn body_id(&self) -> Option<BodyId> {
/* FP:hir.rs-4605 */         match self {
/* FP:hir.rs-4606 */             OwnerNode::Item(Item {
/* FP:hir.rs-4607 */                 kind:
/* FP:hir.rs-4608 */                     ItemKind::Static(_, _, _, body)
/* FP:hir.rs-4609 */                     | ItemKind::Const(_, _, _, body)
/* FP:hir.rs-4610 */                     | ItemKind::Fn { body, .. },
/* FP:hir.rs-4611 */                 ..
/* FP:hir.rs-4612 */             })
/* FP:hir.rs-4613 */             | OwnerNode::TraitItem(TraitItem {
/* FP:hir.rs-4614 */                 kind:
/* FP:hir.rs-4615 */                     TraitItemKind::Fn(_, TraitFn::Provided(body)) | TraitItemKind::Const(_, Some(body)),
/* FP:hir.rs-4616 */                 ..
/* FP:hir.rs-4617 */             })
/* FP:hir.rs-4618 */             | OwnerNode::ImplItem(ImplItem {
/* FP:hir.rs-4619 */                 kind: ImplItemKind::Fn(_, body) | ImplItemKind::Const(_, body),
/* FP:hir.rs-4620 */                 ..
/* FP:hir.rs-4621 */             }) => Some(*body),
/* FP:hir.rs-4622 */             _ => None,
/* FP:hir.rs-4623 */         }
/* FP:hir.rs-4624 */     }
/* FP:hir.rs-4625 */ 
/* FP:hir.rs-4626 */     pub fn generics(self) -> Option<&'hir Generics<'hir>> {
/* FP:hir.rs-4627 */         Node::generics(self.into())
/* FP:hir.rs-4628 */     }
/* FP:hir.rs-4629 */ 
/* FP:hir.rs-4630 */     pub fn def_id(self) -> OwnerId {
/* FP:hir.rs-4631 */         match self {
/* FP:hir.rs-4632 */             OwnerNode::Item(Item { owner_id, .. })
/* FP:hir.rs-4633 */             | OwnerNode::TraitItem(TraitItem { owner_id, .. })
/* FP:hir.rs-4634 */             | OwnerNode::ImplItem(ImplItem { owner_id, .. })
/* FP:hir.rs-4635 */             | OwnerNode::ForeignItem(ForeignItem { owner_id, .. }) => *owner_id,
/* FP:hir.rs-4636 */             OwnerNode::Crate(..) => crate::CRATE_HIR_ID.owner,
/* FP:hir.rs-4637 */             OwnerNode::Synthetic => unreachable!(),
/* FP:hir.rs-4638 */         }
/* FP:hir.rs-4639 */     }
/* FP:hir.rs-4640 */ 
/* FP:hir.rs-4641 */     /// Check if node is an impl block.
/* FP:hir.rs-4642 */     pub fn is_impl_block(&self) -> bool {
/* FP:hir.rs-4643 */         matches!(self, OwnerNode::Item(Item { kind: ItemKind::Impl(_), .. }))
/* FP:hir.rs-4644 */     }
/* FP:hir.rs-4645 */ 
/* FP:hir.rs-4646 */     expect_methods_self! {
/* FP:hir.rs-4647 */         expect_item,         &'hir Item<'hir>,        OwnerNode::Item(n),        n;
/* FP:hir.rs-4648 */         expect_foreign_item, &'hir ForeignItem<'hir>, OwnerNode::ForeignItem(n), n;
/* FP:hir.rs-4649 */         expect_impl_item,    &'hir ImplItem<'hir>,    OwnerNode::ImplItem(n),    n;
/* FP:hir.rs-4650 */         expect_trait_item,   &'hir TraitItem<'hir>,   OwnerNode::TraitItem(n),   n;
/* FP:hir.rs-4651 */     }
/* FP:hir.rs-4652 */ }
/* FP:hir.rs-4653 */ 
/* FP:hir.rs-4654 */ impl<'hir> From<&'hir Item<'hir>> for OwnerNode<'hir> {
/* FP:hir.rs-4655 */     fn from(val: &'hir Item<'hir>) -> Self {
/* FP:hir.rs-4656 */         OwnerNode::Item(val)
/* FP:hir.rs-4657 */     }
/* FP:hir.rs-4658 */ }
/* FP:hir.rs-4659 */ 
/* FP:hir.rs-4660 */ impl<'hir> From<&'hir ForeignItem<'hir>> for OwnerNode<'hir> {
/* FP:hir.rs-4661 */     fn from(val: &'hir ForeignItem<'hir>) -> Self {
/* FP:hir.rs-4662 */         OwnerNode::ForeignItem(val)
/* FP:hir.rs-4663 */     }
/* FP:hir.rs-4664 */ }
/* FP:hir.rs-4665 */ 
/* FP:hir.rs-4666 */ impl<'hir> From<&'hir ImplItem<'hir>> for OwnerNode<'hir> {
/* FP:hir.rs-4667 */     fn from(val: &'hir ImplItem<'hir>) -> Self {
/* FP:hir.rs-4668 */         OwnerNode::ImplItem(val)
/* FP:hir.rs-4669 */     }
/* FP:hir.rs-4670 */ }
/* FP:hir.rs-4671 */ 
/* FP:hir.rs-4672 */ impl<'hir> From<&'hir TraitItem<'hir>> for OwnerNode<'hir> {
/* FP:hir.rs-4673 */     fn from(val: &'hir TraitItem<'hir>) -> Self {
/* FP:hir.rs-4674 */         OwnerNode::TraitItem(val)
/* FP:hir.rs-4675 */     }
/* FP:hir.rs-4676 */ }
/* FP:hir.rs-4677 */ 
/* FP:hir.rs-4678 */ impl<'hir> From<OwnerNode<'hir>> for Node<'hir> {
/* FP:hir.rs-4679 */     fn from(val: OwnerNode<'hir>) -> Self {
/* FP:hir.rs-4680 */         match val {
/* FP:hir.rs-4681 */             OwnerNode::Item(n) => Node::Item(n),
/* FP:hir.rs-4682 */             OwnerNode::ForeignItem(n) => Node::ForeignItem(n),
/* FP:hir.rs-4683 */             OwnerNode::ImplItem(n) => Node::ImplItem(n),
/* FP:hir.rs-4684 */             OwnerNode::TraitItem(n) => Node::TraitItem(n),
/* FP:hir.rs-4685 */             OwnerNode::Crate(n) => Node::Crate(n),
/* FP:hir.rs-4686 */             OwnerNode::Synthetic => Node::Synthetic,
/* FP:hir.rs-4687 */         }
/* FP:hir.rs-4688 */     }
/* FP:hir.rs-4689 */ }
/* FP:hir.rs-4690 */ 
/* FP:hir.rs-4691 */ #[derive(Copy, Clone, Debug, HashStable_Generic)]
/* FP:hir.rs-4692 */ pub enum Node<'hir> {
/* FP:hir.rs-4693 */     Param(&'hir Param<'hir>),
/* FP:hir.rs-4694 */     Item(&'hir Item<'hir>),
/* FP:hir.rs-4695 */     ForeignItem(&'hir ForeignItem<'hir>),
/* FP:hir.rs-4696 */     TraitItem(&'hir TraitItem<'hir>),
/* FP:hir.rs-4697 */     ImplItem(&'hir ImplItem<'hir>),
/* FP:hir.rs-4698 */     Variant(&'hir Variant<'hir>),
/* FP:hir.rs-4699 */     Field(&'hir FieldDef<'hir>),
/* FP:hir.rs-4700 */     AnonConst(&'hir AnonConst),
/* FP:hir.rs-4701 */     ConstBlock(&'hir ConstBlock),
/* FP:hir.rs-4702 */     ConstArg(&'hir ConstArg<'hir>),
/* FP:hir.rs-4703 */     Expr(&'hir Expr<'hir>),
/* FP:hir.rs-4704 */     ExprField(&'hir ExprField<'hir>),
/* FP:hir.rs-4705 */     Stmt(&'hir Stmt<'hir>),
/* FP:hir.rs-4706 */     PathSegment(&'hir PathSegment<'hir>),
/* FP:hir.rs-4707 */     Ty(&'hir Ty<'hir>),
/* FP:hir.rs-4708 */     AssocItemConstraint(&'hir AssocItemConstraint<'hir>),
/* FP:hir.rs-4709 */     TraitRef(&'hir TraitRef<'hir>),
/* FP:hir.rs-4710 */     OpaqueTy(&'hir OpaqueTy<'hir>),
/* FP:hir.rs-4711 */     TyPat(&'hir TyPat<'hir>),
/* FP:hir.rs-4712 */     Pat(&'hir Pat<'hir>),
/* FP:hir.rs-4713 */     PatField(&'hir PatField<'hir>),
/* FP:hir.rs-4714 */     /// Needed as its own node with its own HirId for tracking
/* FP:hir.rs-4715 */     /// the unadjusted type of literals within patterns
/* FP:hir.rs-4716 */     /// (e.g. byte str literals not being of slice type).
/* FP:hir.rs-4717 */     PatExpr(&'hir PatExpr<'hir>),
/* FP:hir.rs-4718 */     Arm(&'hir Arm<'hir>),
/* FP:hir.rs-4719 */     Block(&'hir Block<'hir>),
/* FP:hir.rs-4720 */     LetStmt(&'hir LetStmt<'hir>),
/* FP:hir.rs-4721 */     /// `Ctor` refers to the constructor of an enum variant or struct. Only tuple or unit variants
/* FP:hir.rs-4722 */     /// with synthesized constructors.
/* FP:hir.rs-4723 */     Ctor(&'hir VariantData<'hir>),
/* FP:hir.rs-4724 */     Lifetime(&'hir Lifetime),
/* FP:hir.rs-4725 */     GenericParam(&'hir GenericParam<'hir>),
/* FP:hir.rs-4726 */     Crate(&'hir Mod<'hir>),
/* FP:hir.rs-4727 */     Infer(&'hir InferArg),
/* FP:hir.rs-4728 */     WherePredicate(&'hir WherePredicate<'hir>),
/* FP:hir.rs-4729 */     PreciseCapturingNonLifetimeArg(&'hir PreciseCapturingNonLifetimeArg),
/* FP:hir.rs-4730 */     // Created by query feeding
/* FP:hir.rs-4731 */     Synthetic,
/* FP:hir.rs-4732 */     Err(Span),
/* FP:hir.rs-4733 */ }
/* FP:hir.rs-4734 */ 
/* FP:hir.rs-4735 */ impl<'hir> Node<'hir> {
/* FP:hir.rs-4736 */     /// Get the identifier of this `Node`, if applicable.
/* FP:hir.rs-4737 */     ///
/* FP:hir.rs-4738 */     /// # Edge cases
/* FP:hir.rs-4739 */     ///
/* FP:hir.rs-4740 */     /// Calling `.ident()` on a [`Node::Ctor`] will return `None`
/* FP:hir.rs-4741 */     /// because `Ctor`s do not have identifiers themselves.
/* FP:hir.rs-4742 */     /// Instead, call `.ident()` on the parent struct/variant, like so:
/* FP:hir.rs-4743 */     ///
/* FP:hir.rs-4744 */     /// ```ignore (illustrative)
/* FP:hir.rs-4745 */     /// ctor
/* FP:hir.rs-4746 */     ///     .ctor_hir_id()
/* FP:hir.rs-4747 */     ///     .map(|ctor_id| tcx.parent_hir_node(ctor_id))
/* FP:hir.rs-4748 */     ///     .and_then(|parent| parent.ident())
/* FP:hir.rs-4749 */     /// ```
/* FP:hir.rs-4750 */     pub fn ident(&self) -> Option<Ident> {
/* FP:hir.rs-4751 */         match self {
/* FP:hir.rs-4752 */             Node::Item(item) => item.kind.ident(),
/* FP:hir.rs-4753 */             Node::TraitItem(TraitItem { ident, .. })
/* FP:hir.rs-4754 */             | Node::ImplItem(ImplItem { ident, .. })
/* FP:hir.rs-4755 */             | Node::ForeignItem(ForeignItem { ident, .. })
/* FP:hir.rs-4756 */             | Node::Field(FieldDef { ident, .. })
/* FP:hir.rs-4757 */             | Node::Variant(Variant { ident, .. })
/* FP:hir.rs-4758 */             | Node::PathSegment(PathSegment { ident, .. }) => Some(*ident),
/* FP:hir.rs-4759 */             Node::Lifetime(lt) => Some(lt.ident),
/* FP:hir.rs-4760 */             Node::GenericParam(p) => Some(p.name.ident()),
/* FP:hir.rs-4761 */             Node::AssocItemConstraint(c) => Some(c.ident),
/* FP:hir.rs-4762 */             Node::PatField(f) => Some(f.ident),
/* FP:hir.rs-4763 */             Node::ExprField(f) => Some(f.ident),
/* FP:hir.rs-4764 */             Node::PreciseCapturingNonLifetimeArg(a) => Some(a.ident),
/* FP:hir.rs-4765 */             Node::Param(..)
/* FP:hir.rs-4766 */             | Node::AnonConst(..)
/* FP:hir.rs-4767 */             | Node::ConstBlock(..)
/* FP:hir.rs-4768 */             | Node::ConstArg(..)
/* FP:hir.rs-4769 */             | Node::Expr(..)
/* FP:hir.rs-4770 */             | Node::Stmt(..)
/* FP:hir.rs-4771 */             | Node::Block(..)
/* FP:hir.rs-4772 */             | Node::Ctor(..)
/* FP:hir.rs-4773 */             | Node::Pat(..)
/* FP:hir.rs-4774 */             | Node::TyPat(..)
/* FP:hir.rs-4775 */             | Node::PatExpr(..)
/* FP:hir.rs-4776 */             | Node::Arm(..)
/* FP:hir.rs-4777 */             | Node::LetStmt(..)
/* FP:hir.rs-4778 */             | Node::Crate(..)
/* FP:hir.rs-4779 */             | Node::Ty(..)
/* FP:hir.rs-4780 */             | Node::TraitRef(..)
/* FP:hir.rs-4781 */             | Node::OpaqueTy(..)
/* FP:hir.rs-4782 */             | Node::Infer(..)
/* FP:hir.rs-4783 */             | Node::WherePredicate(..)
/* FP:hir.rs-4784 */             | Node::Synthetic
/* FP:hir.rs-4785 */             | Node::Err(..) => None,
/* FP:hir.rs-4786 */         }
/* FP:hir.rs-4787 */     }
/* FP:hir.rs-4788 */ 
/* FP:hir.rs-4789 */     pub fn fn_decl(self) -> Option<&'hir FnDecl<'hir>> {
/* FP:hir.rs-4790 */         match self {
/* FP:hir.rs-4791 */             Node::TraitItem(TraitItem { kind: TraitItemKind::Fn(fn_sig, _), .. })
/* FP:hir.rs-4792 */             | Node::ImplItem(ImplItem { kind: ImplItemKind::Fn(fn_sig, _), .. })
/* FP:hir.rs-4793 */             | Node::Item(Item { kind: ItemKind::Fn { sig: fn_sig, .. }, .. })
/* FP:hir.rs-4794 */             | Node::ForeignItem(ForeignItem { kind: ForeignItemKind::Fn(fn_sig, _, _), .. }) => {
/* FP:hir.rs-4795 */                 Some(fn_sig.decl)
/* FP:hir.rs-4796 */             }
/* FP:hir.rs-4797 */             Node::Expr(Expr { kind: ExprKind::Closure(Closure { fn_decl, .. }), .. }) => {
/* FP:hir.rs-4798 */                 Some(fn_decl)
/* FP:hir.rs-4799 */             }
/* FP:hir.rs-4800 */             _ => None,
/* FP:hir.rs-4801 */         }
/* FP:hir.rs-4802 */     }
/* FP:hir.rs-4803 */ 
/* FP:hir.rs-4804 */     /// Get a `hir::Impl` if the node is an impl block for the given `trait_def_id`.
/* FP:hir.rs-4805 */     pub fn impl_block_of_trait(self, trait_def_id: DefId) -> Option<&'hir Impl<'hir>> {
/* FP:hir.rs-4806 */         if let Node::Item(Item { kind: ItemKind::Impl(impl_block), .. }) = self
/* FP:hir.rs-4807 */             && let Some(of_trait) = impl_block.of_trait
/* FP:hir.rs-4808 */             && let Some(trait_id) = of_trait.trait_ref.trait_def_id()
/* FP:hir.rs-4809 */             && trait_id == trait_def_id
/* FP:hir.rs-4810 */         {
/* FP:hir.rs-4811 */             Some(impl_block)
/* FP:hir.rs-4812 */         } else {
/* FP:hir.rs-4813 */             None
/* FP:hir.rs-4814 */         }
/* FP:hir.rs-4815 */     }
/* FP:hir.rs-4816 */ 
/* FP:hir.rs-4817 */     pub fn fn_sig(self) -> Option<&'hir FnSig<'hir>> {
/* FP:hir.rs-4818 */         match self {
/* FP:hir.rs-4819 */             Node::TraitItem(TraitItem { kind: TraitItemKind::Fn(fn_sig, _), .. })
/* FP:hir.rs-4820 */             | Node::ImplItem(ImplItem { kind: ImplItemKind::Fn(fn_sig, _), .. })
/* FP:hir.rs-4821 */             | Node::Item(Item { kind: ItemKind::Fn { sig: fn_sig, .. }, .. })
/* FP:hir.rs-4822 */             | Node::ForeignItem(ForeignItem { kind: ForeignItemKind::Fn(fn_sig, _, _), .. }) => {
/* FP:hir.rs-4823 */                 Some(fn_sig)
/* FP:hir.rs-4824 */             }
/* FP:hir.rs-4825 */             _ => None,
/* FP:hir.rs-4826 */         }
/* FP:hir.rs-4827 */     }
/* FP:hir.rs-4828 */ 
/* FP:hir.rs-4829 */     /// Get the type for constants, assoc types, type aliases and statics.
/* FP:hir.rs-4830 */     pub fn ty(self) -> Option<&'hir Ty<'hir>> {
/* FP:hir.rs-4831 */         match self {
/* FP:hir.rs-4832 */             Node::Item(it) => match it.kind {
/* FP:hir.rs-4833 */                 ItemKind::TyAlias(_, _, ty)
/* FP:hir.rs-4834 */                 | ItemKind::Static(_, _, ty, _)
/* FP:hir.rs-4835 */                 | ItemKind::Const(_, _, ty, _) => Some(ty),
/* FP:hir.rs-4836 */                 ItemKind::Impl(impl_item) => Some(&impl_item.self_ty),
/* FP:hir.rs-4837 */                 _ => None,
/* FP:hir.rs-4838 */             },
/* FP:hir.rs-4839 */             Node::TraitItem(it) => match it.kind {
/* FP:hir.rs-4840 */                 TraitItemKind::Const(ty, _) => Some(ty),
/* FP:hir.rs-4841 */                 TraitItemKind::Type(_, ty) => ty,
/* FP:hir.rs-4842 */                 _ => None,
/* FP:hir.rs-4843 */             },
/* FP:hir.rs-4844 */             Node::ImplItem(it) => match it.kind {
/* FP:hir.rs-4845 */                 ImplItemKind::Const(ty, _) => Some(ty),
/* FP:hir.rs-4846 */                 ImplItemKind::Type(ty) => Some(ty),
/* FP:hir.rs-4847 */                 _ => None,
/* FP:hir.rs-4848 */             },
/* FP:hir.rs-4849 */             Node::ForeignItem(it) => match it.kind {
/* FP:hir.rs-4850 */                 ForeignItemKind::Static(ty, ..) => Some(ty),
/* FP:hir.rs-4851 */                 _ => None,
/* FP:hir.rs-4852 */             },
/* FP:hir.rs-4853 */             _ => None,
/* FP:hir.rs-4854 */         }
/* FP:hir.rs-4855 */     }
/* FP:hir.rs-4856 */ 
/* FP:hir.rs-4857 */     pub fn alias_ty(self) -> Option<&'hir Ty<'hir>> {
/* FP:hir.rs-4858 */         match self {
/* FP:hir.rs-4859 */             Node::Item(Item { kind: ItemKind::TyAlias(_, _, ty), .. }) => Some(ty),
/* FP:hir.rs-4860 */             _ => None,
/* FP:hir.rs-4861 */         }
/* FP:hir.rs-4862 */     }
/* FP:hir.rs-4863 */ 
/* FP:hir.rs-4864 */     #[inline]
/* FP:hir.rs-4865 */     pub fn associated_body(&self) -> Option<(LocalDefId, BodyId)> {
/* FP:hir.rs-4866 */         match self {
/* FP:hir.rs-4867 */             Node::Item(Item {
/* FP:hir.rs-4868 */                 owner_id,
/* FP:hir.rs-4869 */                 kind:
/* FP:hir.rs-4870 */                     ItemKind::Const(_, _, _, body)
/* FP:hir.rs-4871 */                     | ItemKind::Static(.., body)
/* FP:hir.rs-4872 */                     | ItemKind::Fn { body, .. },
/* FP:hir.rs-4873 */                 ..
/* FP:hir.rs-4874 */             })
/* FP:hir.rs-4875 */             | Node::TraitItem(TraitItem {
/* FP:hir.rs-4876 */                 owner_id,
/* FP:hir.rs-4877 */                 kind:
/* FP:hir.rs-4878 */                     TraitItemKind::Const(_, Some(body)) | TraitItemKind::Fn(_, TraitFn::Provided(body)),
/* FP:hir.rs-4879 */                 ..
/* FP:hir.rs-4880 */             })
/* FP:hir.rs-4881 */             | Node::ImplItem(ImplItem {
/* FP:hir.rs-4882 */                 owner_id,
/* FP:hir.rs-4883 */                 kind: ImplItemKind::Const(_, body) | ImplItemKind::Fn(_, body),
/* FP:hir.rs-4884 */                 ..
/* FP:hir.rs-4885 */             }) => Some((owner_id.def_id, *body)),
/* FP:hir.rs-4886 */ 
/* FP:hir.rs-4887 */             Node::Item(Item {
/* FP:hir.rs-4888 */                 owner_id, kind: ItemKind::GlobalAsm { asm: _, fake_body }, ..
/* FP:hir.rs-4889 */             }) => Some((owner_id.def_id, *fake_body)),
/* FP:hir.rs-4890 */ 
/* FP:hir.rs-4891 */             Node::Expr(Expr { kind: ExprKind::Closure(Closure { def_id, body, .. }), .. }) => {
/* FP:hir.rs-4892 */                 Some((*def_id, *body))
/* FP:hir.rs-4893 */             }
/* FP:hir.rs-4894 */ 
/* FP:hir.rs-4895 */             Node::AnonConst(constant) => Some((constant.def_id, constant.body)),
/* FP:hir.rs-4896 */             Node::ConstBlock(constant) => Some((constant.def_id, constant.body)),
/* FP:hir.rs-4897 */ 
/* FP:hir.rs-4898 */             _ => None,
/* FP:hir.rs-4899 */         }
/* FP:hir.rs-4900 */     }
/* FP:hir.rs-4901 */ 
/* FP:hir.rs-4902 */     pub fn body_id(&self) -> Option<BodyId> {
/* FP:hir.rs-4903 */         Some(self.associated_body()?.1)
/* FP:hir.rs-4904 */     }
/* FP:hir.rs-4905 */ 
/* FP:hir.rs-4906 */     pub fn generics(self) -> Option<&'hir Generics<'hir>> {
/* FP:hir.rs-4907 */         match self {
/* FP:hir.rs-4908 */             Node::ForeignItem(ForeignItem {
/* FP:hir.rs-4909 */                 kind: ForeignItemKind::Fn(_, _, generics), ..
/* FP:hir.rs-4910 */             })
/* FP:hir.rs-4911 */             | Node::TraitItem(TraitItem { generics, .. })
/* FP:hir.rs-4912 */             | Node::ImplItem(ImplItem { generics, .. }) => Some(generics),
/* FP:hir.rs-4913 */             Node::Item(item) => item.kind.generics(),
/* FP:hir.rs-4914 */             _ => None,
/* FP:hir.rs-4915 */         }
/* FP:hir.rs-4916 */     }
/* FP:hir.rs-4917 */ 
/* FP:hir.rs-4918 */     pub fn as_owner(self) -> Option<OwnerNode<'hir>> {
/* FP:hir.rs-4919 */         match self {
/* FP:hir.rs-4920 */             Node::Item(i) => Some(OwnerNode::Item(i)),
/* FP:hir.rs-4921 */             Node::ForeignItem(i) => Some(OwnerNode::ForeignItem(i)),
/* FP:hir.rs-4922 */             Node::TraitItem(i) => Some(OwnerNode::TraitItem(i)),
/* FP:hir.rs-4923 */             Node::ImplItem(i) => Some(OwnerNode::ImplItem(i)),
/* FP:hir.rs-4924 */             Node::Crate(i) => Some(OwnerNode::Crate(i)),
/* FP:hir.rs-4925 */             Node::Synthetic => Some(OwnerNode::Synthetic),
/* FP:hir.rs-4926 */             _ => None,
/* FP:hir.rs-4927 */         }
/* FP:hir.rs-4928 */     }
/* FP:hir.rs-4929 */ 
/* FP:hir.rs-4930 */     pub fn fn_kind(self) -> Option<FnKind<'hir>> {
/* FP:hir.rs-4931 */         match self {
/* FP:hir.rs-4932 */             Node::Item(i) => match i.kind {
/* FP:hir.rs-4933 */                 ItemKind::Fn { ident, sig, generics, .. } => {
/* FP:hir.rs-4934 */                     Some(FnKind::ItemFn(ident, generics, sig.header))
/* FP:hir.rs-4935 */                 }
/* FP:hir.rs-4936 */                 _ => None,
/* FP:hir.rs-4937 */             },
/* FP:hir.rs-4938 */             Node::TraitItem(ti) => match ti.kind {
/* FP:hir.rs-4939 */                 TraitItemKind::Fn(ref sig, _) => Some(FnKind::Method(ti.ident, sig)),
/* FP:hir.rs-4940 */                 _ => None,
/* FP:hir.rs-4941 */             },
/* FP:hir.rs-4942 */             Node::ImplItem(ii) => match ii.kind {
/* FP:hir.rs-4943 */                 ImplItemKind::Fn(ref sig, _) => Some(FnKind::Method(ii.ident, sig)),
/* FP:hir.rs-4944 */                 _ => None,
/* FP:hir.rs-4945 */             },
/* FP:hir.rs-4946 */             Node::Expr(e) => match e.kind {
/* FP:hir.rs-4947 */                 ExprKind::Closure { .. } => Some(FnKind::Closure),
/* FP:hir.rs-4948 */                 _ => None,
/* FP:hir.rs-4949 */             },
/* FP:hir.rs-4950 */             _ => None,
/* FP:hir.rs-4951 */         }
/* FP:hir.rs-4952 */     }
/* FP:hir.rs-4953 */ 
/* FP:hir.rs-4954 */     expect_methods_self! {
/* FP:hir.rs-4955 */         expect_param,         &'hir Param<'hir>,        Node::Param(n),        n;
/* FP:hir.rs-4956 */         expect_item,          &'hir Item<'hir>,         Node::Item(n),         n;
/* FP:hir.rs-4957 */         expect_foreign_item,  &'hir ForeignItem<'hir>,  Node::ForeignItem(n),  n;
/* FP:hir.rs-4958 */         expect_trait_item,    &'hir TraitItem<'hir>,    Node::TraitItem(n),    n;
/* FP:hir.rs-4959 */         expect_impl_item,     &'hir ImplItem<'hir>,     Node::ImplItem(n),     n;
/* FP:hir.rs-4960 */         expect_variant,       &'hir Variant<'hir>,      Node::Variant(n),      n;
/* FP:hir.rs-4961 */         expect_field,         &'hir FieldDef<'hir>,     Node::Field(n),        n;
/* FP:hir.rs-4962 */         expect_anon_const,    &'hir AnonConst,          Node::AnonConst(n),    n;
/* FP:hir.rs-4963 */         expect_inline_const,  &'hir ConstBlock,         Node::ConstBlock(n),   n;
/* FP:hir.rs-4964 */         expect_expr,          &'hir Expr<'hir>,         Node::Expr(n),         n;
/* FP:hir.rs-4965 */         expect_expr_field,    &'hir ExprField<'hir>,    Node::ExprField(n),    n;
/* FP:hir.rs-4966 */         expect_stmt,          &'hir Stmt<'hir>,         Node::Stmt(n),         n;
/* FP:hir.rs-4967 */         expect_path_segment,  &'hir PathSegment<'hir>,  Node::PathSegment(n),  n;
/* FP:hir.rs-4968 */         expect_ty,            &'hir Ty<'hir>,           Node::Ty(n),           n;
/* FP:hir.rs-4969 */         expect_assoc_item_constraint,  &'hir AssocItemConstraint<'hir>,  Node::AssocItemConstraint(n),  n;
/* FP:hir.rs-4970 */         expect_trait_ref,     &'hir TraitRef<'hir>,     Node::TraitRef(n),     n;
/* FP:hir.rs-4971 */         expect_opaque_ty,     &'hir OpaqueTy<'hir>,     Node::OpaqueTy(n),     n;
/* FP:hir.rs-4972 */         expect_pat,           &'hir Pat<'hir>,          Node::Pat(n),          n;
/* FP:hir.rs-4973 */         expect_pat_field,     &'hir PatField<'hir>,     Node::PatField(n),     n;
/* FP:hir.rs-4974 */         expect_arm,           &'hir Arm<'hir>,          Node::Arm(n),          n;
/* FP:hir.rs-4975 */         expect_block,         &'hir Block<'hir>,        Node::Block(n),        n;
/* FP:hir.rs-4976 */         expect_let_stmt,      &'hir LetStmt<'hir>,      Node::LetStmt(n),      n;
/* FP:hir.rs-4977 */         expect_ctor,          &'hir VariantData<'hir>,  Node::Ctor(n),         n;
/* FP:hir.rs-4978 */         expect_lifetime,      &'hir Lifetime,           Node::Lifetime(n),     n;
/* FP:hir.rs-4979 */         expect_generic_param, &'hir GenericParam<'hir>, Node::GenericParam(n), n;
/* FP:hir.rs-4980 */         expect_crate,         &'hir Mod<'hir>,          Node::Crate(n),        n;
/* FP:hir.rs-4981 */         expect_infer,         &'hir InferArg,           Node::Infer(n),        n;
/* FP:hir.rs-4982 */         expect_closure,       &'hir Closure<'hir>, Node::Expr(Expr { kind: ExprKind::Closure(n), .. }), n;
/* FP:hir.rs-4983 */     }
/* FP:hir.rs-4984 */ }
/* FP:hir.rs-4985 */ 
/* FP:hir.rs-4986 */ // Some nodes are used a lot. Make sure they don't unintentionally get bigger.
/* FP:hir.rs-4987 */ #[cfg(target_pointer_width = "64")]
/* FP:hir.rs-4988 */ mod size_asserts {
/* FP:hir.rs-4989 */     use crate::rustc_data_structures::static_assert_size;
/* FP:hir.rs-4990 */ 
/* FP:hir.rs-4991 */     use super::*;
/* FP:hir.rs-4992 */     // tidy-alphabetical-start
/* FP:hir.rs-4993 */     static_assert_size!(Block<'_>, 48);
/* FP:hir.rs-4994 */     static_assert_size!(Body<'_>, 24);
/* FP:hir.rs-4995 */     static_assert_size!(Expr<'_>, 64);
/* FP:hir.rs-4996 */     static_assert_size!(ExprKind<'_>, 48);
/* FP:hir.rs-4997 */     static_assert_size!(FnDecl<'_>, 40);
/* FP:hir.rs-4998 */     static_assert_size!(ForeignItem<'_>, 96);
/* FP:hir.rs-4999 */     static_assert_size!(ForeignItemKind<'_>, 56);
/* FP:hir.rs-5000 */     static_assert_size!(GenericArg<'_>, 16);
/* FP:hir.rs-5001 */     static_assert_size!(GenericBound<'_>, 64);
/* FP:hir.rs-5002 */     static_assert_size!(Generics<'_>, 56);
/* FP:hir.rs-5003 */     static_assert_size!(Impl<'_>, 40);
/* FP:hir.rs-5004 */     static_assert_size!(ImplItem<'_>, 88);
/* FP:hir.rs-5005 */     static_assert_size!(ImplItemKind<'_>, 40);
/* FP:hir.rs-5006 */     static_assert_size!(Item<'_>, 88);
/* FP:hir.rs-5007 */     static_assert_size!(ItemKind<'_>, 64);
/* FP:hir.rs-5008 */     static_assert_size!(LetStmt<'_>, 72);
/* FP:hir.rs-5009 */     static_assert_size!(Param<'_>, 32);
/* FP:hir.rs-5010 */     static_assert_size!(Pat<'_>, 80);
/* FP:hir.rs-5011 */     static_assert_size!(PatKind<'_>, 56);
/* FP:hir.rs-5012 */     static_assert_size!(Path<'_>, 40);
/* FP:hir.rs-5013 */     static_assert_size!(PathSegment<'_>, 48);
/* FP:hir.rs-5014 */     static_assert_size!(QPath<'_>, 24);
/* FP:hir.rs-5015 */     static_assert_size!(Res, 12);
/* FP:hir.rs-5016 */     static_assert_size!(Stmt<'_>, 32);
/* FP:hir.rs-5017 */     static_assert_size!(StmtKind<'_>, 16);
/* FP:hir.rs-5018 */     static_assert_size!(TraitImplHeader<'_>, 48);
/* FP:hir.rs-5019 */     static_assert_size!(TraitItem<'_>, 88);
/* FP:hir.rs-5020 */     static_assert_size!(TraitItemKind<'_>, 48);
/* FP:hir.rs-5021 */     static_assert_size!(Ty<'_>, 48);
/* FP:hir.rs-5022 */     static_assert_size!(TyKind<'_>, 32);
/* FP:hir.rs-5023 */     // tidy-alphabetical-end
/* FP:hir.rs-5024 */ }
/* FP:hir.rs-5025 */ 
/* FP:hir.rs-5026 */ #[cfg(test)]