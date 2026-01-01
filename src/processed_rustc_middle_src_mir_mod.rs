/* FP:mod.rs-0001 */ // MIR datatypes and passes. See the [rustc dev guide] for more info.
/* FP:mod.rs-0002 */ //
/* FP:mod.rs-0003 */ // [rustc dev guide]: https://rustc-dev-guide.rust-lang.org/mir/index.html
/* FP:mod.rs-0004 */ 
/* FP:mod.rs-0005 */ use std::borrow::Cow;
/* FP:mod.rs-0006 */ use std::fmt::{self, Debug, Formatter};
/* FP:mod.rs-0007 */ use std::iter;
/* FP:mod.rs-0008 */ use std::ops::{Index, IndexMut};
/* FP:mod.rs-0009 */ 
/* FP:mod.rs-0010 */ pub use basic_blocks::{BasicBlocks, SwitchTargetValue};
/* FP:mod.rs-0011 */ use either::Either;
/* FP:mod.rs-0012 */ use polonius_engine::Atom;
/* FP:mod.rs-0013 */ use crate::rustc_abi::{FieldIdx, VariantIdx};
/* FP:mod.rs-0014 */ pub use crate::rustc_complete::Mutability;
/* FP:mod.rs-0015 */ use crate::rustc_data_structures::fx::{FxHashMap, FxHashSet};
/* FP:mod.rs-0016 */ use crate::rustc_data_structures::graph::dominators::Dominators;
/* FP:mod.rs-0017 */ use crate::rustc_complete::{DiagArgName, DiagArgValue, DiagMessage, ErrorGuaranteed, IntoDiagArg};
/* FP:mod.rs-0018 */ use crate::rustc_complete::def::{CtorKind, Namespace};
/* FP:mod.rs-0019 */ use crate::rustc_complete::def_id::{CRATE_DEF_ID, DefId};
/* FP:mod.rs-0020 */ use crate::rustc_complete::{
/* FP:mod.rs-0021 */     self as hir, BindingMode, ByRef, CoroutineDesugaring, CoroutineKind, HirId, ImplicitSelfKind,
/* FP:mod.rs-0022 */ };
/* FP:mod.rs-0023 */ use crate::rustc_index::bit_set::DenseBitSet;
/* FP:mod.rs-0024 */ use crate::rustc_index::{Idx, IndexSlice, IndexVec};
/* FP:mod.rs-0025 */ use rustc_macros::{HashStable, TyDecodable, TyEncodable, TypeFoldable, TypeVisitable};
/* FP:mod.rs-0026 */ use crate::rustc_serialize::{Decodable, Encodable};
/* FP:mod.rs-0027 */ use crate::rustc_complete::source_map::Spanned;
/* FP:mod.rs-0028 */ use crate::rustc_complete::{DUMMY_SP, Span, Symbol};
/* FP:mod.rs-0029 */ use tracing::{debug, trace};
/* FP:mod.rs-0030 */ 
/* FP:mod.rs-0031 */ pub use self::query::*;
/* FP:mod.rs-0032 */ use crate::mir::interpret::{AllocRange, Scalar};
/* FP:mod.rs-0033 */ use crate::ty::codec::{TyDecoder, TyEncoder};
/* FP:mod.rs-0034 */ use crate::ty::print::{FmtPrinter, Printer, pretty_print_const, with_no_trimmed_paths};
/* FP:mod.rs-0035 */ use crate::ty::{
/* FP:mod.rs-0036 */     self, GenericArg, GenericArgsRef, Instance, InstanceKind, List, Ty, TyCtxt, TypeVisitableExt,
/* FP:mod.rs-0037 */     TypingEnv, UserTypeAnnotationIndex,
/* FP:mod.rs-0038 */ };
/* FP:mod.rs-0039 */ 
/* FP:mod.rs-0053 */ 
/* FP:mod.rs-0057 */ 
/* FP:mod.rs-0058 */ pub use consts::*;
/* FP:mod.rs-0059 */ use pretty::pretty_print_const_value;
/* FP:mod.rs-0060 */ pub use statement::*;
/* FP:mod.rs-0061 */ pub use syntax::*;
/* FP:mod.rs-0062 */ pub use terminator::*;
/* FP:mod.rs-0063 */ 
/* FP:mod.rs-0064 */ pub use self::generic_graph::graphviz_safe_def_name;
/* FP:mod.rs-0065 */ pub use self::graphviz::write_mir_graphviz;
/* FP:mod.rs-0066 */ pub use self::pretty::{MirDumper, PassWhere, display_allocation, write_mir_pretty};
/* FP:mod.rs-0067 */ 
/* FP:mod.rs-0068 */ /// Types for locals
/* FP:mod.rs-0069 */ pub type LocalDecls<'tcx> = IndexSlice<Local, LocalDecl<'tcx>>;
/* FP:mod.rs-0070 */ 
/* FP:mod.rs-0071 */ pub trait HasLocalDecls<'tcx> {
/* FP:mod.rs-0072 */     fn local_decls(&self) -> &LocalDecls<'tcx>;
/* FP:mod.rs-0073 */ }
/* FP:mod.rs-0074 */ 
/* FP:mod.rs-0075 */ impl<'tcx> HasLocalDecls<'tcx> for IndexVec<Local, LocalDecl<'tcx>> {
/* FP:mod.rs-0076 */     #[inline]
/* FP:mod.rs-0077 */     fn local_decls(&self) -> &LocalDecls<'tcx> {
/* FP:mod.rs-0078 */         self
/* FP:mod.rs-0079 */     }
/* FP:mod.rs-0080 */ }
/* FP:mod.rs-0081 */ 
/* FP:mod.rs-0082 */ impl<'tcx> HasLocalDecls<'tcx> for LocalDecls<'tcx> {
/* FP:mod.rs-0083 */     #[inline]
/* FP:mod.rs-0084 */     fn local_decls(&self) -> &LocalDecls<'tcx> {
/* FP:mod.rs-0085 */         self
/* FP:mod.rs-0086 */     }
/* FP:mod.rs-0087 */ }
/* FP:mod.rs-0088 */ 
/* FP:mod.rs-0089 */ impl<'tcx> HasLocalDecls<'tcx> for Body<'tcx> {
/* FP:mod.rs-0090 */     #[inline]
/* FP:mod.rs-0091 */     fn local_decls(&self) -> &LocalDecls<'tcx> {
/* FP:mod.rs-0092 */         &self.local_decls
/* FP:mod.rs-0093 */     }
/* FP:mod.rs-0094 */ }
/* FP:mod.rs-0095 */ 
/* FP:mod.rs-0096 */ impl MirPhase {
/* FP:mod.rs-0097 */     pub fn name(&self) -> &'static str {
/* FP:mod.rs-0098 */         match *self {
/* FP:mod.rs-0099 */             MirPhase::Built => "built",
/* FP:mod.rs-0100 */             MirPhase::Analysis(AnalysisPhase::Initial) => "analysis",
/* FP:mod.rs-0101 */             MirPhase::Analysis(AnalysisPhase::PostCleanup) => "analysis-post-cleanup",
/* FP:mod.rs-0102 */             MirPhase::Runtime(RuntimePhase::Initial) => "runtime",
/* FP:mod.rs-0103 */             MirPhase::Runtime(RuntimePhase::PostCleanup) => "runtime-post-cleanup",
/* FP:mod.rs-0104 */             MirPhase::Runtime(RuntimePhase::Optimized) => "runtime-optimized",
/* FP:mod.rs-0105 */         }
/* FP:mod.rs-0106 */     }
/* FP:mod.rs-0107 */ 
/* FP:mod.rs-0108 */     /// Gets the (dialect, phase) index of the current `MirPhase`. Both numbers
/* FP:mod.rs-0109 */     /// are 1-indexed.
/* FP:mod.rs-0110 */     pub fn index(&self) -> (usize, usize) {
/* FP:mod.rs-0111 */         match *self {
/* FP:mod.rs-0112 */             MirPhase::Built => (1, 1),
/* FP:mod.rs-0113 */             MirPhase::Analysis(analysis_phase) => (2, 1 + analysis_phase as usize),
/* FP:mod.rs-0114 */             MirPhase::Runtime(runtime_phase) => (3, 1 + runtime_phase as usize),
/* FP:mod.rs-0115 */         }
/* FP:mod.rs-0116 */     }
/* FP:mod.rs-0117 */ }
/* FP:mod.rs-0118 */ 
/* FP:mod.rs-0119 */ /// Where a specific `mir::Body` comes from.
/* FP:mod.rs-0120 */ #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/* FP:mod.rs-0121 */ #[derive(HashStable, TyEncodable, TyDecodable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-0122 */ pub struct MirSource<'tcx> {
/* FP:mod.rs-0123 */     pub instance: InstanceKind<'tcx>,
/* FP:mod.rs-0124 */ 
/* FP:mod.rs-0125 */     /// If `Some`, this is a promoted rvalue within the parent function.
/* FP:mod.rs-0126 */     pub promoted: Option<Promoted>,
/* FP:mod.rs-0127 */ }
/* FP:mod.rs-0128 */ 
/* FP:mod.rs-0129 */ impl<'tcx> MirSource<'tcx> {
/* FP:mod.rs-0130 */     pub fn item(def_id: DefId) -> Self {
/* FP:mod.rs-0131 */         MirSource { instance: InstanceKind::Item(def_id), promoted: None }
/* FP:mod.rs-0132 */     }
/* FP:mod.rs-0133 */ 
/* FP:mod.rs-0134 */     pub fn from_instance(instance: InstanceKind<'tcx>) -> Self {
/* FP:mod.rs-0135 */         MirSource { instance, promoted: None }
/* FP:mod.rs-0136 */     }
/* FP:mod.rs-0137 */ 
/* FP:mod.rs-0138 */     #[inline]
/* FP:mod.rs-0139 */     pub fn def_id(&self) -> DefId {
/* FP:mod.rs-0140 */         self.instance.def_id()
/* FP:mod.rs-0141 */     }
/* FP:mod.rs-0142 */ }
/* FP:mod.rs-0143 */ 
/* FP:mod.rs-0144 */ /// Additional information carried by a MIR body when it is lowered from a coroutine.
/* FP:mod.rs-0145 */ /// This information is modified as it is lowered during the `StateTransform` MIR pass,
/* FP:mod.rs-0146 */ /// so not all fields will be active at a given time. For example, the `yield_ty` is
/* FP:mod.rs-0147 */ /// taken out of the field after yields are turned into returns, and the `coroutine_drop`
/* FP:mod.rs-0148 */ /// body is only populated after the state transform pass.
/* FP:mod.rs-0149 */ #[derive(Clone, TyEncodable, TyDecodable, Debug, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-0150 */ pub struct CoroutineInfo<'tcx> {
/* FP:mod.rs-0151 */     /// The yield type of the function. This field is removed after the state transform pass.
/* FP:mod.rs-0152 */     pub yield_ty: Option<Ty<'tcx>>,
/* FP:mod.rs-0153 */ 
/* FP:mod.rs-0154 */     /// The resume type of the function. This field is removed after the state transform pass.
/* FP:mod.rs-0155 */     pub resume_ty: Option<Ty<'tcx>>,
/* FP:mod.rs-0156 */ 
/* FP:mod.rs-0157 */     /// Coroutine drop glue. This field is populated after the state transform pass.
/* FP:mod.rs-0158 */     pub coroutine_drop: Option<Body<'tcx>>,
/* FP:mod.rs-0159 */ 
/* FP:mod.rs-0160 */     /// Coroutine async drop glue.
/* FP:mod.rs-0161 */     pub coroutine_drop_async: Option<Body<'tcx>>,
/* FP:mod.rs-0162 */ 
/* FP:mod.rs-0163 */     /// When coroutine has sync drop, this is async proxy calling `coroutine_drop` sync impl.
/* FP:mod.rs-0164 */     pub coroutine_drop_proxy_async: Option<Body<'tcx>>,
/* FP:mod.rs-0165 */ 
/* FP:mod.rs-0166 */     /// The layout of a coroutine. Produced by the state transformation.
/* FP:mod.rs-0167 */     pub coroutine_layout: Option<CoroutineLayout<'tcx>>,
/* FP:mod.rs-0168 */ 
/* FP:mod.rs-0169 */     /// If this is a coroutine then record the type of source expression that caused this coroutine
/* FP:mod.rs-0170 */     /// to be created.
/* FP:mod.rs-0171 */     pub coroutine_kind: CoroutineKind,
/* FP:mod.rs-0172 */ }
/* FP:mod.rs-0173 */ 
/* FP:mod.rs-0174 */ impl<'tcx> CoroutineInfo<'tcx> {
/* FP:mod.rs-0175 */     // Sets up `CoroutineInfo` for a pre-coroutine-transform MIR body.
/* FP:mod.rs-0176 */     pub fn initial(
/* FP:mod.rs-0177 */         coroutine_kind: CoroutineKind,
/* FP:mod.rs-0178 */         yield_ty: Ty<'tcx>,
/* FP:mod.rs-0179 */         resume_ty: Ty<'tcx>,
/* FP:mod.rs-0180 */     ) -> CoroutineInfo<'tcx> {
/* FP:mod.rs-0181 */         CoroutineInfo {
/* FP:mod.rs-0182 */             coroutine_kind,
/* FP:mod.rs-0183 */             yield_ty: Some(yield_ty),
/* FP:mod.rs-0184 */             resume_ty: Some(resume_ty),
/* FP:mod.rs-0185 */             coroutine_drop: None,
/* FP:mod.rs-0186 */             coroutine_drop_async: None,
/* FP:mod.rs-0187 */             coroutine_drop_proxy_async: None,
/* FP:mod.rs-0188 */             coroutine_layout: None,
/* FP:mod.rs-0189 */         }
/* FP:mod.rs-0190 */     }
/* FP:mod.rs-0191 */ }
/* FP:mod.rs-0192 */ 
/* FP:mod.rs-0193 */ /// Some item that needs to monomorphize successfully for a MIR body to be considered well-formed.
/* FP:mod.rs-0194 */ #[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, HashStable, TyEncodable, TyDecodable)]
/* FP:mod.rs-0195 */ #[derive(TypeFoldable, TypeVisitable)]
/* FP:mod.rs-0196 */ pub enum MentionedItem<'tcx> {
/* FP:mod.rs-0197 */     /// A function that gets called. We don't necessarily know its precise type yet, since it can be
/* FP:mod.rs-0198 */     /// hidden behind a generic.
/* FP:mod.rs-0199 */     Fn(Ty<'tcx>),
/* FP:mod.rs-0200 */     /// A type that has its drop shim called.
/* FP:mod.rs-0201 */     Drop(Ty<'tcx>),
/* FP:mod.rs-0202 */     /// Unsizing casts might require vtables, so we have to record them.
/* FP:mod.rs-0203 */     UnsizeCast { source_ty: Ty<'tcx>, target_ty: Ty<'tcx> },
/* FP:mod.rs-0204 */     /// A closure that is coerced to a function pointer.
/* FP:mod.rs-0205 */     Closure(Ty<'tcx>),
/* FP:mod.rs-0206 */ }
/* FP:mod.rs-0207 */ 
/* FP:mod.rs-0208 */ /// The lowered representation of a single function.
/* FP:mod.rs-0209 */ #[derive(Clone, TyEncodable, TyDecodable, Debug, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-0210 */ pub struct Body<'tcx> {
/* FP:mod.rs-0211 */     /// A list of basic blocks. References to basic block use a newtyped index type [`BasicBlock`]
/* FP:mod.rs-0212 */     /// that indexes into this vector.
/* FP:mod.rs-0213 */     pub basic_blocks: BasicBlocks<'tcx>,
/* FP:mod.rs-0214 */ 
/* FP:mod.rs-0215 */     /// Records how far through the "desugaring and optimization" process this particular
/* FP:mod.rs-0216 */     /// MIR has traversed. This is particularly useful when inlining, since in that context
/* FP:mod.rs-0217 */     /// we instantiate the promoted constants and add them to our promoted vector -- but those
/* FP:mod.rs-0218 */     /// promoted items have already been optimized, whereas ours have not. This field allows
/* FP:mod.rs-0219 */     /// us to see the difference and forego optimization on the inlined promoted items.
/* FP:mod.rs-0220 */     pub phase: MirPhase,
/* FP:mod.rs-0221 */ 
/* FP:mod.rs-0222 */     /// How many passes we have executed since starting the current phase. Used for debug output.
/* FP:mod.rs-0223 */     pub pass_count: usize,
/* FP:mod.rs-0224 */ 
/* FP:mod.rs-0225 */     pub source: MirSource<'tcx>,
/* FP:mod.rs-0226 */ 
/* FP:mod.rs-0227 */     /// A list of source scopes; these are referenced by statements
/* FP:mod.rs-0228 */     /// and used for debuginfo. Indexed by a `SourceScope`.
/* FP:mod.rs-0229 */     pub source_scopes: IndexVec<SourceScope, SourceScopeData<'tcx>>,
/* FP:mod.rs-0230 */ 
/* FP:mod.rs-0231 */     /// Additional information carried by a MIR body when it is lowered from a coroutine.
/* FP:mod.rs-0232 */     ///
/* FP:mod.rs-0233 */     /// Note that the coroutine drop shim, any promoted consts, and other synthetic MIR
/* FP:mod.rs-0234 */     /// bodies that come from processing a coroutine body are not typically coroutines
/* FP:mod.rs-0235 */     /// themselves, and should probably set this to `None` to avoid carrying redundant
/* FP:mod.rs-0236 */     /// information.
/* FP:mod.rs-0237 */     pub coroutine: Option<Box<CoroutineInfo<'tcx>>>,
/* FP:mod.rs-0238 */ 
/* FP:mod.rs-0239 */     /// Declarations of locals.
/* FP:mod.rs-0240 */     ///
/* FP:mod.rs-0241 */     /// The first local is the return value pointer, followed by `arg_count`
/* FP:mod.rs-0242 */     /// locals for the function arguments, followed by any user-declared
/* FP:mod.rs-0243 */     /// variables and temporaries.
/* FP:mod.rs-0244 */     pub local_decls: IndexVec<Local, LocalDecl<'tcx>>,
/* FP:mod.rs-0245 */ 
/* FP:mod.rs-0246 */     /// User type annotations.
/* FP:mod.rs-0247 */     pub user_type_annotations: ty::CanonicalUserTypeAnnotations<'tcx>,
/* FP:mod.rs-0248 */ 
/* FP:mod.rs-0249 */     /// The number of arguments this function takes.
/* FP:mod.rs-0250 */     ///
/* FP:mod.rs-0251 */     /// Starting at local 1, `arg_count` locals will be provided by the caller
/* FP:mod.rs-0252 */     /// and can be assumed to be initialized.
/* FP:mod.rs-0253 */     ///
/* FP:mod.rs-0254 */     /// If this MIR was built for a constant, this will be 0.
/* FP:mod.rs-0255 */     pub arg_count: usize,
/* FP:mod.rs-0256 */ 
/* FP:mod.rs-0257 */     /// Mark an argument local (which must be a tuple) as getting passed as
/* FP:mod.rs-0258 */     /// its individual components at the LLVM level.
/* FP:mod.rs-0259 */     ///
/* FP:mod.rs-0260 */     /// This is used for the "rust-call" ABI.
/* FP:mod.rs-0261 */     pub spread_arg: Option<Local>,
/* FP:mod.rs-0262 */ 
/* FP:mod.rs-0263 */     /// Debug information pertaining to user variables, including captures.
/* FP:mod.rs-0264 */     pub var_debug_info: Vec<VarDebugInfo<'tcx>>,
/* FP:mod.rs-0265 */ 
/* FP:mod.rs-0266 */     /// A span representing this MIR, for error reporting.
/* FP:mod.rs-0267 */     pub span: Span,
/* FP:mod.rs-0268 */ 
/* FP:mod.rs-0269 */     /// Constants that are required to evaluate successfully for this MIR to be well-formed.
/* FP:mod.rs-0270 */     /// We hold in this field all the constants we are not able to evaluate yet.
/* FP:mod.rs-0271 */     /// `None` indicates that the list has not been computed yet.
/* FP:mod.rs-0272 */     ///
/* FP:mod.rs-0273 */     /// This is soundness-critical, we make a guarantee that all consts syntactically mentioned in a
/* FP:mod.rs-0274 */     /// function have successfully evaluated if the function ever gets executed at runtime.
/* FP:mod.rs-0275 */     pub required_consts: Option<Vec<ConstOperand<'tcx>>>,
/* FP:mod.rs-0276 */ 
/* FP:mod.rs-0277 */     /// Further items that were mentioned in this function and hence *may* become monomorphized,
/* FP:mod.rs-0278 */     /// depending on optimizations. We use this to avoid optimization-dependent compile errors: the
/* FP:mod.rs-0279 */     /// collector recursively traverses all "mentioned" items and evaluates all their
/* FP:mod.rs-0280 */     /// `required_consts`.
/* FP:mod.rs-0281 */     /// `None` indicates that the list has not been computed yet.
/* FP:mod.rs-0282 */     ///
/* FP:mod.rs-0283 */     /// This is *not* soundness-critical and the contents of this list are *not* a stable guarantee.
/* FP:mod.rs-0284 */     /// All that's relevant is that this set is optimization-level-independent, and that it includes
/* FP:mod.rs-0285 */     /// everything that the collector would consider "used". (For example, we currently compute this
/* FP:mod.rs-0286 */     /// set after drop elaboration, so some drop calls that can never be reached are not considered
/* FP:mod.rs-0287 */     /// "mentioned".) See the documentation of `CollectionMode` in
/* FP:mod.rs-0288 */     /// `compiler/rustc_monomorphize/src/collector.rs` for more context.
/* FP:mod.rs-0289 */     pub mentioned_items: Option<Vec<Spanned<MentionedItem<'tcx>>>>,
/* FP:mod.rs-0290 */ 
/* FP:mod.rs-0291 */     /// Does this body use generic parameters. This is used for the `ConstEvaluatable` check.
/* FP:mod.rs-0292 */     ///
/* FP:mod.rs-0293 */     /// Note that this does not actually mean that this body is not computable right now.
/* FP:mod.rs-0294 */     /// The repeat count in the following example is polymorphic, but can still be evaluated
/* FP:mod.rs-0295 */     /// without knowing anything about the type parameter `T`.
/* FP:mod.rs-0296 */     ///
/* FP:mod.rs-0297 */     /// ```rust
/* FP:mod.rs-0298 */     /// fn test<T>() {
/* FP:mod.rs-0299 */     ///     let _ = [0; size_of::<*mut T>()];
/* FP:mod.rs-0300 */     /// }
/* FP:mod.rs-0301 */     /// ```
/* FP:mod.rs-0302 */     ///
/* FP:mod.rs-0303 */     /// **WARNING**: Do not change this flags after the MIR was originally created, even if an optimization
/* FP:mod.rs-0304 */     /// removed the last mention of all generic params. We do not want to rely on optimizations and
/* FP:mod.rs-0305 */     /// potentially allow things like `[u8; size_of::<T>() * 0]` due to this.
/* FP:mod.rs-0306 */     pub is_polymorphic: bool,
/* FP:mod.rs-0307 */ 
/* FP:mod.rs-0308 */     /// The phase at which this MIR should be "injected" into the compilation process.
/* FP:mod.rs-0309 */     ///
/* FP:mod.rs-0310 */     /// Everything that comes before this `MirPhase` should be skipped.
/* FP:mod.rs-0311 */     ///
/* FP:mod.rs-0312 */     /// This is only `Some` if the function that this body comes from was annotated with `rustc_custom_mir`.
/* FP:mod.rs-0313 */     pub injection_phase: Option<MirPhase>,
/* FP:mod.rs-0314 */ 
/* FP:mod.rs-0315 */     pub tainted_by_errors: Option<ErrorGuaranteed>,
/* FP:mod.rs-0316 */ 
/* FP:mod.rs-0317 */     /// Coverage information collected from THIR/MIR during MIR building,
/* FP:mod.rs-0318 */     /// to be used by the `InstrumentCoverage` pass.
/* FP:mod.rs-0319 */     ///
/* FP:mod.rs-0320 */     /// Only present if coverage is enabled and this function is eligible.
/* FP:mod.rs-0321 */     /// Boxed to limit space overhead in non-coverage builds.
/* FP:mod.rs-0322 */     #[type_foldable(identity)]
/* FP:mod.rs-0323 */     #[type_visitable(ignore)]
/* FP:mod.rs-0324 */     pub coverage_info_hi: Option<Box<coverage::CoverageInfoHi>>,
/* FP:mod.rs-0325 */ 
/* FP:mod.rs-0326 */     /// Per-function coverage information added by the `InstrumentCoverage`
/* FP:mod.rs-0327 */     /// pass, to be used in conjunction with the coverage statements injected
/* FP:mod.rs-0328 */     /// into this body's blocks.
/* FP:mod.rs-0329 */     ///
/* FP:mod.rs-0330 */     /// If `-Cinstrument-coverage` is not active, or if an individual function
/* FP:mod.rs-0331 */     /// is not eligible for coverage, then this should always be `None`.
/* FP:mod.rs-0332 */     #[type_foldable(identity)]
/* FP:mod.rs-0333 */     #[type_visitable(ignore)]
/* FP:mod.rs-0334 */     pub function_coverage_info: Option<Box<coverage::FunctionCoverageInfo>>,
/* FP:mod.rs-0335 */ }
/* FP:mod.rs-0336 */ 
/* FP:mod.rs-0337 */ impl<'tcx> Body<'tcx> {
/* FP:mod.rs-0338 */     pub fn new(
/* FP:mod.rs-0339 */         source: MirSource<'tcx>,
/* FP:mod.rs-0340 */         basic_blocks: IndexVec<BasicBlock, BasicBlockData<'tcx>>,
/* FP:mod.rs-0341 */         source_scopes: IndexVec<SourceScope, SourceScopeData<'tcx>>,
/* FP:mod.rs-0342 */         local_decls: IndexVec<Local, LocalDecl<'tcx>>,
/* FP:mod.rs-0343 */         user_type_annotations: ty::CanonicalUserTypeAnnotations<'tcx>,
/* FP:mod.rs-0344 */         arg_count: usize,
/* FP:mod.rs-0345 */         var_debug_info: Vec<VarDebugInfo<'tcx>>,
/* FP:mod.rs-0346 */         span: Span,
/* FP:mod.rs-0347 */         coroutine: Option<Box<CoroutineInfo<'tcx>>>,
/* FP:mod.rs-0348 */         tainted_by_errors: Option<ErrorGuaranteed>,
/* FP:mod.rs-0349 */     ) -> Self {
/* FP:mod.rs-0350 */         // We need `arg_count` locals, and one for the return place.
/* FP:mod.rs-0351 */         assert!(
/* FP:mod.rs-0352 */             local_decls.len() > arg_count,
/* FP:mod.rs-0353 */             "expected at least {} locals, got {}",
/* FP:mod.rs-0354 */             arg_count + 1,
/* FP:mod.rs-0355 */             local_decls.len()
/* FP:mod.rs-0356 */         );
/* FP:mod.rs-0357 */ 
/* FP:mod.rs-0358 */         let mut body = Body {
/* FP:mod.rs-0359 */             phase: MirPhase::Built,
/* FP:mod.rs-0360 */             pass_count: 0,
/* FP:mod.rs-0361 */             source,
/* FP:mod.rs-0362 */             basic_blocks: BasicBlocks::new(basic_blocks),
/* FP:mod.rs-0363 */             source_scopes,
/* FP:mod.rs-0364 */             coroutine,
/* FP:mod.rs-0365 */             local_decls,
/* FP:mod.rs-0366 */             user_type_annotations,
/* FP:mod.rs-0367 */             arg_count,
/* FP:mod.rs-0368 */             spread_arg: None,
/* FP:mod.rs-0369 */             var_debug_info,
/* FP:mod.rs-0370 */             span,
/* FP:mod.rs-0371 */             required_consts: None,
/* FP:mod.rs-0372 */             mentioned_items: None,
/* FP:mod.rs-0373 */             is_polymorphic: false,
/* FP:mod.rs-0374 */             injection_phase: None,
/* FP:mod.rs-0375 */             tainted_by_errors,
/* FP:mod.rs-0376 */             coverage_info_hi: None,
/* FP:mod.rs-0377 */             function_coverage_info: None,
/* FP:mod.rs-0378 */         };
/* FP:mod.rs-0379 */         body.is_polymorphic = body.has_non_region_param();
/* FP:mod.rs-0380 */         body
/* FP:mod.rs-0381 */     }
/* FP:mod.rs-0382 */ 
/* FP:mod.rs-0383 */     /// Returns a partially initialized MIR body containing only a list of basic blocks.
/* FP:mod.rs-0384 */     ///
/* FP:mod.rs-0385 */     /// The returned MIR contains no `LocalDecl`s (even for the return place) or source scopes. It
/* FP:mod.rs-0386 */     /// is only useful for testing but cannot be `#[cfg(test)]` because it is used in a different
/* FP:mod.rs-0387 */     /// crate.
/* FP:mod.rs-0388 */     pub fn new_cfg_only(basic_blocks: IndexVec<BasicBlock, BasicBlockData<'tcx>>) -> Self {
/* FP:mod.rs-0389 */         let mut body = Body {
/* FP:mod.rs-0390 */             phase: MirPhase::Built,
/* FP:mod.rs-0391 */             pass_count: 0,
/* FP:mod.rs-0392 */             source: MirSource::item(CRATE_DEF_ID.to_def_id()),
/* FP:mod.rs-0393 */             basic_blocks: BasicBlocks::new(basic_blocks),
/* FP:mod.rs-0394 */             source_scopes: IndexVec::new(),
/* FP:mod.rs-0395 */             coroutine: None,
/* FP:mod.rs-0396 */             local_decls: IndexVec::new(),
/* FP:mod.rs-0397 */             user_type_annotations: IndexVec::new(),
/* FP:mod.rs-0398 */             arg_count: 0,
/* FP:mod.rs-0399 */             spread_arg: None,
/* FP:mod.rs-0400 */             span: DUMMY_SP,
/* FP:mod.rs-0401 */             required_consts: None,
/* FP:mod.rs-0402 */             mentioned_items: None,
/* FP:mod.rs-0403 */             var_debug_info: Vec::new(),
/* FP:mod.rs-0404 */             is_polymorphic: false,
/* FP:mod.rs-0405 */             injection_phase: None,
/* FP:mod.rs-0406 */             tainted_by_errors: None,
/* FP:mod.rs-0407 */             coverage_info_hi: None,
/* FP:mod.rs-0408 */             function_coverage_info: None,
/* FP:mod.rs-0409 */         };
/* FP:mod.rs-0410 */         body.is_polymorphic = body.has_non_region_param();
/* FP:mod.rs-0411 */         body
/* FP:mod.rs-0412 */     }
/* FP:mod.rs-0413 */ 
/* FP:mod.rs-0414 */     #[inline]
/* FP:mod.rs-0415 */     pub fn basic_blocks_mut(&mut self) -> &mut IndexVec<BasicBlock, BasicBlockData<'tcx>> {
/* FP:mod.rs-0416 */         self.basic_blocks.as_mut()
/* FP:mod.rs-0417 */     }
/* FP:mod.rs-0418 */ 
/* FP:mod.rs-0419 */     pub fn typing_env(&self, tcx: TyCtxt<'tcx>) -> TypingEnv<'tcx> {
/* FP:mod.rs-0420 */         match self.phase {
/* FP:mod.rs-0421 */             // FIXME(#132279): we should reveal the opaques defined in the body during analysis.
/* FP:mod.rs-0422 */             MirPhase::Built | MirPhase::Analysis(_) => TypingEnv {
/* FP:mod.rs-0423 */                 typing_mode: ty::TypingMode::non_body_analysis(),
/* FP:mod.rs-0424 */                 param_env: tcx.param_env(self.source.def_id()),
/* FP:mod.rs-0425 */             },
/* FP:mod.rs-0426 */             MirPhase::Runtime(_) => TypingEnv::post_analysis(tcx, self.source.def_id()),
/* FP:mod.rs-0427 */         }
/* FP:mod.rs-0428 */     }
/* FP:mod.rs-0429 */ 
/* FP:mod.rs-0430 */     #[inline]
/* FP:mod.rs-0431 */     pub fn local_kind(&self, local: Local) -> LocalKind {
/* FP:mod.rs-0432 */         let index = local.as_usize();
/* FP:mod.rs-0433 */         if index == 0 {
/* FP:mod.rs-0434 */             debug_assert!(
/* FP:mod.rs-0435 */                 self.local_decls[local].mutability == Mutability::Mut,
/* FP:mod.rs-0436 */                 "return place should be mutable"
/* FP:mod.rs-0437 */             );
/* FP:mod.rs-0438 */ 
/* FP:mod.rs-0439 */             LocalKind::ReturnPointer
/* FP:mod.rs-0440 */         } else if index < self.arg_count + 1 {
/* FP:mod.rs-0441 */             LocalKind::Arg
/* FP:mod.rs-0442 */         } else {
/* FP:mod.rs-0443 */             LocalKind::Temp
/* FP:mod.rs-0444 */         }
/* FP:mod.rs-0445 */     }
/* FP:mod.rs-0446 */ 
/* FP:mod.rs-0447 */     /// Returns an iterator over all user-declared mutable locals.
/* FP:mod.rs-0448 */     #[inline]
/* FP:mod.rs-0449 */     pub fn mut_vars_iter(&self) -> impl Iterator<Item = Local> {
/* FP:mod.rs-0450 */         (self.arg_count + 1..self.local_decls.len()).filter_map(move |index| {
/* FP:mod.rs-0451 */             let local = Local::new(index);
/* FP:mod.rs-0452 */             let decl = &self.local_decls[local];
/* FP:mod.rs-0453 */             (decl.is_user_variable() && decl.mutability.is_mut()).then_some(local)
/* FP:mod.rs-0454 */         })
/* FP:mod.rs-0455 */     }
/* FP:mod.rs-0456 */ 
/* FP:mod.rs-0457 */     /// Returns an iterator over all user-declared mutable arguments and locals.
/* FP:mod.rs-0458 */     #[inline]
/* FP:mod.rs-0459 */     pub fn mut_vars_and_args_iter(&self) -> impl Iterator<Item = Local> {
/* FP:mod.rs-0460 */         (1..self.local_decls.len()).filter_map(move |index| {
/* FP:mod.rs-0461 */             let local = Local::new(index);
/* FP:mod.rs-0462 */             let decl = &self.local_decls[local];
/* FP:mod.rs-0463 */             if (decl.is_user_variable() || index < self.arg_count + 1)
/* FP:mod.rs-0464 */                 && decl.mutability == Mutability::Mut
/* FP:mod.rs-0465 */             {
/* FP:mod.rs-0466 */                 Some(local)
/* FP:mod.rs-0467 */             } else {
/* FP:mod.rs-0468 */                 None
/* FP:mod.rs-0469 */             }
/* FP:mod.rs-0470 */         })
/* FP:mod.rs-0471 */     }
/* FP:mod.rs-0472 */ 
/* FP:mod.rs-0473 */     /// Returns an iterator over all function arguments.
/* FP:mod.rs-0474 */     #[inline]
/* FP:mod.rs-0475 */     pub fn args_iter(&self) -> impl Iterator<Item = Local> + ExactSizeIterator {
/* FP:mod.rs-0476 */         (1..self.arg_count + 1).map(Local::new)
/* FP:mod.rs-0477 */     }
/* FP:mod.rs-0478 */ 
/* FP:mod.rs-0479 */     /// Returns an iterator over all user-defined variables and compiler-generated temporaries (all
/* FP:mod.rs-0480 */     /// locals that are neither arguments nor the return place).
/* FP:mod.rs-0481 */     #[inline]
/* FP:mod.rs-0482 */     pub fn vars_and_temps_iter(
/* FP:mod.rs-0483 */         &self,
/* FP:mod.rs-0484 */     ) -> impl DoubleEndedIterator<Item = Local> + ExactSizeIterator {
/* FP:mod.rs-0485 */         (self.arg_count + 1..self.local_decls.len()).map(Local::new)
/* FP:mod.rs-0486 */     }
/* FP:mod.rs-0487 */ 
/* FP:mod.rs-0488 */     #[inline]
/* FP:mod.rs-0489 */     pub fn drain_vars_and_temps(&mut self) -> impl Iterator<Item = LocalDecl<'tcx>> {
/* FP:mod.rs-0490 */         self.local_decls.drain(self.arg_count + 1..)
/* FP:mod.rs-0491 */     }
/* FP:mod.rs-0492 */ 
/* FP:mod.rs-0493 */     /// Returns the source info associated with `location`.
/* FP:mod.rs-0494 */     pub fn source_info(&self, location: Location) -> &SourceInfo {
/* FP:mod.rs-0495 */         let block = &self[location.block];
/* FP:mod.rs-0496 */         let stmts = &block.statements;
/* FP:mod.rs-0497 */         let idx = location.statement_index;
/* FP:mod.rs-0498 */         if idx < stmts.len() {
/* FP:mod.rs-0499 */             &stmts[idx].source_info
/* FP:mod.rs-0500 */         } else {
/* FP:mod.rs-0501 */             assert_eq!(idx, stmts.len());
/* FP:mod.rs-0502 */             &block.terminator().source_info
/* FP:mod.rs-0503 */         }
/* FP:mod.rs-0504 */     }
/* FP:mod.rs-0505 */ 
/* FP:mod.rs-0506 */     /// Returns the return type; it always return first element from `local_decls` array.
/* FP:mod.rs-0507 */     #[inline]
/* FP:mod.rs-0508 */     pub fn return_ty(&self) -> Ty<'tcx> {
/* FP:mod.rs-0509 */         self.local_decls[RETURN_PLACE].ty
/* FP:mod.rs-0510 */     }
/* FP:mod.rs-0511 */ 
/* FP:mod.rs-0512 */     /// Returns the return type; it always return first element from `local_decls` array.
/* FP:mod.rs-0513 */     #[inline]
/* FP:mod.rs-0514 */     pub fn bound_return_ty(&self) -> ty::EarlyBinder<'tcx, Ty<'tcx>> {
/* FP:mod.rs-0515 */         ty::EarlyBinder::bind(self.local_decls[RETURN_PLACE].ty)
/* FP:mod.rs-0516 */     }
/* FP:mod.rs-0517 */ 
/* FP:mod.rs-0518 */     /// Gets the location of the terminator for the given block.
/* FP:mod.rs-0519 */     #[inline]
/* FP:mod.rs-0520 */     pub fn terminator_loc(&self, bb: BasicBlock) -> Location {
/* FP:mod.rs-0521 */         Location { block: bb, statement_index: self[bb].statements.len() }
/* FP:mod.rs-0522 */     }
/* FP:mod.rs-0523 */ 
/* FP:mod.rs-0524 */     pub fn stmt_at(&self, location: Location) -> Either<&Statement<'tcx>, &Terminator<'tcx>> {
/* FP:mod.rs-0525 */         let Location { block, statement_index } = location;
/* FP:mod.rs-0526 */         let block_data = &self.basic_blocks[block];
/* FP:mod.rs-0527 */         block_data
/* FP:mod.rs-0528 */             .statements
/* FP:mod.rs-0529 */             .get(statement_index)
/* FP:mod.rs-0530 */             .map(Either::Left)
/* FP:mod.rs-0531 */             .unwrap_or_else(|| Either::Right(block_data.terminator()))
/* FP:mod.rs-0532 */     }
/* FP:mod.rs-0533 */ 
/* FP:mod.rs-0534 */     #[inline]
/* FP:mod.rs-0535 */     pub fn yield_ty(&self) -> Option<Ty<'tcx>> {
/* FP:mod.rs-0536 */         self.coroutine.as_ref().and_then(|coroutine| coroutine.yield_ty)
/* FP:mod.rs-0537 */     }
/* FP:mod.rs-0538 */ 
/* FP:mod.rs-0539 */     #[inline]
/* FP:mod.rs-0540 */     pub fn resume_ty(&self) -> Option<Ty<'tcx>> {
/* FP:mod.rs-0541 */         self.coroutine.as_ref().and_then(|coroutine| coroutine.resume_ty)
/* FP:mod.rs-0542 */     }
/* FP:mod.rs-0543 */ 
/* FP:mod.rs-0544 */     /// Prefer going through [`TyCtxt::coroutine_layout`] rather than using this directly.
/* FP:mod.rs-0545 */     #[inline]
/* FP:mod.rs-0546 */     pub fn coroutine_layout_raw(&self) -> Option<&CoroutineLayout<'tcx>> {
/* FP:mod.rs-0547 */         self.coroutine.as_ref().and_then(|coroutine| coroutine.coroutine_layout.as_ref())
/* FP:mod.rs-0548 */     }
/* FP:mod.rs-0549 */ 
/* FP:mod.rs-0550 */     #[inline]
/* FP:mod.rs-0551 */     pub fn coroutine_drop(&self) -> Option<&Body<'tcx>> {
/* FP:mod.rs-0552 */         self.coroutine.as_ref().and_then(|coroutine| coroutine.coroutine_drop.as_ref())
/* FP:mod.rs-0553 */     }
/* FP:mod.rs-0554 */ 
/* FP:mod.rs-0555 */     #[inline]
/* FP:mod.rs-0556 */     pub fn coroutine_drop_async(&self) -> Option<&Body<'tcx>> {
/* FP:mod.rs-0557 */         self.coroutine.as_ref().and_then(|coroutine| coroutine.coroutine_drop_async.as_ref())
/* FP:mod.rs-0558 */     }
/* FP:mod.rs-0559 */ 
/* FP:mod.rs-0560 */     #[inline]
/* FP:mod.rs-0561 */     pub fn coroutine_requires_async_drop(&self) -> bool {
/* FP:mod.rs-0562 */         self.coroutine_drop_async().is_some()
/* FP:mod.rs-0563 */     }
/* FP:mod.rs-0564 */ 
/* FP:mod.rs-0565 */     #[inline]
/* FP:mod.rs-0566 */     pub fn future_drop_poll(&self) -> Option<&Body<'tcx>> {
/* FP:mod.rs-0567 */         self.coroutine.as_ref().and_then(|coroutine| {
/* FP:mod.rs-0568 */             coroutine
/* FP:mod.rs-0569 */                 .coroutine_drop_async
/* FP:mod.rs-0570 */                 .as_ref()
/* FP:mod.rs-0571 */                 .or(coroutine.coroutine_drop_proxy_async.as_ref())
/* FP:mod.rs-0572 */         })
/* FP:mod.rs-0573 */     }
/* FP:mod.rs-0574 */ 
/* FP:mod.rs-0575 */     #[inline]
/* FP:mod.rs-0576 */     pub fn coroutine_kind(&self) -> Option<CoroutineKind> {
/* FP:mod.rs-0577 */         self.coroutine.as_ref().map(|coroutine| coroutine.coroutine_kind)
/* FP:mod.rs-0578 */     }
/* FP:mod.rs-0579 */ 
/* FP:mod.rs-0580 */     #[inline]
/* FP:mod.rs-0581 */     pub fn should_skip(&self) -> bool {
/* FP:mod.rs-0582 */         let Some(injection_phase) = self.injection_phase else {
/* FP:mod.rs-0583 */             return false;
/* FP:mod.rs-0584 */         };
/* FP:mod.rs-0585 */         injection_phase > self.phase
/* FP:mod.rs-0586 */     }
/* FP:mod.rs-0587 */ 
/* FP:mod.rs-0588 */     #[inline]
/* FP:mod.rs-0589 */     pub fn is_custom_mir(&self) -> bool {
/* FP:mod.rs-0590 */         self.injection_phase.is_some()
/* FP:mod.rs-0591 */     }
/* FP:mod.rs-0592 */ 
/* FP:mod.rs-0593 */     /// If this basic block ends with a [`TerminatorKind::SwitchInt`] for which we can evaluate the
/* FP:mod.rs-0594 */     /// discriminant in monomorphization, we return the discriminant bits and the
/* FP:mod.rs-0595 */     /// [`SwitchTargets`], just so the caller doesn't also have to match on the terminator.
/* FP:mod.rs-0596 */     fn try_const_mono_switchint<'a>(
/* FP:mod.rs-0597 */         tcx: TyCtxt<'tcx>,
/* FP:mod.rs-0598 */         instance: Instance<'tcx>,
/* FP:mod.rs-0599 */         block: &'a BasicBlockData<'tcx>,
/* FP:mod.rs-0600 */     ) -> Option<(u128, &'a SwitchTargets)> {
/* FP:mod.rs-0601 */         // There are two places here we need to evaluate a constant.
/* FP:mod.rs-0602 */         let eval_mono_const = |constant: &ConstOperand<'tcx>| {
/* FP:mod.rs-0603 */             // FIXME(#132279): what is this, why are we using an empty environment here.
/* FP:mod.rs-0604 */             let typing_env = ty::TypingEnv::fully_monomorphized();
/* FP:mod.rs-0605 */             let mono_literal = instance.instantiate_mir_and_normalize_erasing_regions(
/* FP:mod.rs-0606 */                 tcx,
/* FP:mod.rs-0607 */                 typing_env,
/* FP:mod.rs-0608 */                 crate::ty::EarlyBinder::bind(constant.const_),
/* FP:mod.rs-0609 */             );
/* FP:mod.rs-0610 */             mono_literal.try_eval_bits(tcx, typing_env)
/* FP:mod.rs-0611 */         };
/* FP:mod.rs-0612 */ 
/* FP:mod.rs-0613 */         let TerminatorKind::SwitchInt { discr, targets } = &block.terminator().kind else {
/* FP:mod.rs-0614 */             return None;
/* FP:mod.rs-0615 */         };
/* FP:mod.rs-0616 */ 
/* FP:mod.rs-0617 */         // If this is a SwitchInt(const _), then we can just evaluate the constant and return.
/* FP:mod.rs-0618 */         let discr = match discr {
/* FP:mod.rs-0619 */             Operand::Constant(constant) => {
/* FP:mod.rs-0620 */                 let bits = eval_mono_const(constant)?;
/* FP:mod.rs-0621 */                 return Some((bits, targets));
/* FP:mod.rs-0622 */             }
/* FP:mod.rs-0623 */             Operand::Move(place) | Operand::Copy(place) => place,
/* FP:mod.rs-0624 */         };
/* FP:mod.rs-0625 */ 
/* FP:mod.rs-0626 */         // MIR for `if false` actually looks like this:
/* FP:mod.rs-0627 */         // _1 = const _
/* FP:mod.rs-0628 */         // SwitchInt(_1)
/* FP:mod.rs-0629 */         //
/* FP:mod.rs-0630 */         // And MIR for if intrinsics::ub_checks() looks like this:
/* FP:mod.rs-0631 */         // _1 = UbChecks()
/* FP:mod.rs-0632 */         // SwitchInt(_1)
/* FP:mod.rs-0633 */         //
/* FP:mod.rs-0634 */         // So we're going to try to recognize this pattern.
/* FP:mod.rs-0635 */         //
/* FP:mod.rs-0636 */         // If we have a SwitchInt on a non-const place, we find the most recent statement that
/* FP:mod.rs-0637 */         // isn't a storage marker. If that statement is an assignment of a const to our
/* FP:mod.rs-0638 */         // discriminant place, we evaluate and return the const, as if we've const-propagated it
/* FP:mod.rs-0639 */         // into the SwitchInt.
/* FP:mod.rs-0640 */ 
/* FP:mod.rs-0641 */         let last_stmt = block.statements.iter().rev().find(|stmt| {
/* FP:mod.rs-0642 */             !matches!(stmt.kind, StatementKind::StorageDead(_) | StatementKind::StorageLive(_))
/* FP:mod.rs-0643 */         })?;
/* FP:mod.rs-0644 */ 
/* FP:mod.rs-0645 */         let (place, rvalue) = last_stmt.kind.as_assign()?;
/* FP:mod.rs-0646 */ 
/* FP:mod.rs-0647 */         if discr != place {
/* FP:mod.rs-0648 */             return None;
/* FP:mod.rs-0649 */         }
/* FP:mod.rs-0650 */ 
/* FP:mod.rs-0651 */         match rvalue {
/* FP:mod.rs-0652 */             Rvalue::NullaryOp(NullOp::UbChecks, _) => Some((tcx.sess.ub_checks() as u128, targets)),
/* FP:mod.rs-0653 */             Rvalue::Use(Operand::Constant(constant)) => {
/* FP:mod.rs-0654 */                 let bits = eval_mono_const(constant)?;
/* FP:mod.rs-0655 */                 Some((bits, targets))
/* FP:mod.rs-0656 */             }
/* FP:mod.rs-0657 */             _ => None,
/* FP:mod.rs-0658 */         }
/* FP:mod.rs-0659 */     }
/* FP:mod.rs-0660 */ 
/* FP:mod.rs-0661 */     /// For a `Location` in this scope, determine what the "caller location" at that point is. This
/* FP:mod.rs-0662 */     /// is interesting because of inlining: the `#[track_caller]` attribute of inlined functions
/* FP:mod.rs-0663 */     /// must be honored. Falls back to the `tracked_caller` value for `#[track_caller]` functions,
/* FP:mod.rs-0664 */     /// or the function's scope.
/* FP:mod.rs-0665 */     pub fn caller_location_span<T>(
/* FP:mod.rs-0666 */         &self,
/* FP:mod.rs-0667 */         mut source_info: SourceInfo,
/* FP:mod.rs-0668 */         caller_location: Option<T>,
/* FP:mod.rs-0669 */         tcx: TyCtxt<'tcx>,
/* FP:mod.rs-0670 */         from_span: impl FnOnce(Span) -> T,
/* FP:mod.rs-0671 */     ) -> T {
/* FP:mod.rs-0672 */         loop {
/* FP:mod.rs-0673 */             let scope_data = &self.source_scopes[source_info.scope];
/* FP:mod.rs-0674 */ 
/* FP:mod.rs-0675 */             if let Some((callee, callsite_span)) = scope_data.inlined {
/* FP:mod.rs-0676 */                 // Stop inside the most nested non-`#[track_caller]` function,
/* FP:mod.rs-0677 */                 // before ever reaching its caller (which is irrelevant).
/* FP:mod.rs-0678 */                 if !callee.def.requires_caller_location(tcx) {
/* FP:mod.rs-0679 */                     return from_span(source_info.span);
/* FP:mod.rs-0680 */                 }
/* FP:mod.rs-0681 */                 source_info.span = callsite_span;
/* FP:mod.rs-0682 */             }
/* FP:mod.rs-0683 */ 
/* FP:mod.rs-0684 */             // Skip past all of the parents with `inlined: None`.
/* FP:mod.rs-0685 */             match scope_data.inlined_parent_scope {
/* FP:mod.rs-0686 */                 Some(parent) => source_info.scope = parent,
/* FP:mod.rs-0687 */                 None => break,
/* FP:mod.rs-0688 */             }
/* FP:mod.rs-0689 */         }
/* FP:mod.rs-0690 */ 
/* FP:mod.rs-0691 */         // No inlined `SourceScope`s, or all of them were `#[track_caller]`.
/* FP:mod.rs-0692 */         caller_location.unwrap_or_else(|| from_span(source_info.span))
/* FP:mod.rs-0693 */     }
/* FP:mod.rs-0694 */ 
/* FP:mod.rs-0695 */     #[track_caller]
/* FP:mod.rs-0696 */     pub fn set_required_consts(&mut self, required_consts: Vec<ConstOperand<'tcx>>) {
/* FP:mod.rs-0697 */         assert!(
/* FP:mod.rs-0698 */             self.required_consts.is_none(),
/* FP:mod.rs-0699 */             "required_consts for {:?} have already been set",
/* FP:mod.rs-0700 */             self.source.def_id()
/* FP:mod.rs-0701 */         );
/* FP:mod.rs-0702 */         self.required_consts = Some(required_consts);
/* FP:mod.rs-0703 */     }
/* FP:mod.rs-0704 */     #[track_caller]
/* FP:mod.rs-0705 */     pub fn required_consts(&self) -> &[ConstOperand<'tcx>] {
/* FP:mod.rs-0706 */         match &self.required_consts {
/* FP:mod.rs-0707 */             Some(l) => l,
/* FP:mod.rs-0708 */             None => panic!("required_consts for {:?} have not yet been set", self.source.def_id()),
/* FP:mod.rs-0709 */         }
/* FP:mod.rs-0710 */     }
/* FP:mod.rs-0711 */ 
/* FP:mod.rs-0712 */     #[track_caller]
/* FP:mod.rs-0713 */     pub fn set_mentioned_items(&mut self, mentioned_items: Vec<Spanned<MentionedItem<'tcx>>>) {
/* FP:mod.rs-0714 */         assert!(
/* FP:mod.rs-0715 */             self.mentioned_items.is_none(),
/* FP:mod.rs-0716 */             "mentioned_items for {:?} have already been set",
/* FP:mod.rs-0717 */             self.source.def_id()
/* FP:mod.rs-0718 */         );
/* FP:mod.rs-0719 */         self.mentioned_items = Some(mentioned_items);
/* FP:mod.rs-0720 */     }
/* FP:mod.rs-0721 */     #[track_caller]
/* FP:mod.rs-0722 */     pub fn mentioned_items(&self) -> &[Spanned<MentionedItem<'tcx>>] {
/* FP:mod.rs-0723 */         match &self.mentioned_items {
/* FP:mod.rs-0724 */             Some(l) => l,
/* FP:mod.rs-0725 */             None => panic!("mentioned_items for {:?} have not yet been set", self.source.def_id()),
/* FP:mod.rs-0726 */         }
/* FP:mod.rs-0727 */     }
/* FP:mod.rs-0728 */ }
/* FP:mod.rs-0729 */ 
/* FP:mod.rs-0730 */ impl<'tcx> Index<BasicBlock> for Body<'tcx> {
/* FP:mod.rs-0731 */     type Output = BasicBlockData<'tcx>;
/* FP:mod.rs-0732 */ 
/* FP:mod.rs-0733 */     #[inline]
/* FP:mod.rs-0734 */     fn index(&self, index: BasicBlock) -> &BasicBlockData<'tcx> {
/* FP:mod.rs-0735 */         &self.basic_blocks[index]
/* FP:mod.rs-0736 */     }
/* FP:mod.rs-0737 */ }
/* FP:mod.rs-0738 */ 
/* FP:mod.rs-0739 */ impl<'tcx> IndexMut<BasicBlock> for Body<'tcx> {
/* FP:mod.rs-0740 */     #[inline]
/* FP:mod.rs-0741 */     fn index_mut(&mut self, index: BasicBlock) -> &mut BasicBlockData<'tcx> {
/* FP:mod.rs-0742 */         &mut self.basic_blocks.as_mut()[index]
/* FP:mod.rs-0743 */     }
/* FP:mod.rs-0744 */ }
/* FP:mod.rs-0745 */ 
/* FP:mod.rs-0746 */ #[derive(Copy, Clone, Debug, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-0747 */ pub enum ClearCrossCrate<T> {
/* FP:mod.rs-0748 */     Clear,
/* FP:mod.rs-0749 */     Set(T),
/* FP:mod.rs-0750 */ }
/* FP:mod.rs-0751 */ 
/* FP:mod.rs-0752 */ impl<T> ClearCrossCrate<T> {
/* FP:mod.rs-0753 */     pub fn as_ref(&self) -> ClearCrossCrate<&T> {
/* FP:mod.rs-0754 */         match self {
/* FP:mod.rs-0755 */             ClearCrossCrate::Clear => ClearCrossCrate::Clear,
/* FP:mod.rs-0756 */             ClearCrossCrate::Set(v) => ClearCrossCrate::Set(v),
/* FP:mod.rs-0757 */         }
/* FP:mod.rs-0758 */     }
/* FP:mod.rs-0759 */ 
/* FP:mod.rs-0760 */     pub fn as_mut(&mut self) -> ClearCrossCrate<&mut T> {
/* FP:mod.rs-0761 */         match self {
/* FP:mod.rs-0762 */             ClearCrossCrate::Clear => ClearCrossCrate::Clear,
/* FP:mod.rs-0763 */             ClearCrossCrate::Set(v) => ClearCrossCrate::Set(v),
/* FP:mod.rs-0764 */         }
/* FP:mod.rs-0765 */     }
/* FP:mod.rs-0766 */ 
/* FP:mod.rs-0767 */     pub fn unwrap_crate_local(self) -> T {
/* FP:mod.rs-0768 */         match self {
/* FP:mod.rs-0769 */             ClearCrossCrate::Clear => bug!("unwrapping cross-crate data"),
/* FP:mod.rs-0770 */             ClearCrossCrate::Set(v) => v,
/* FP:mod.rs-0771 */         }
/* FP:mod.rs-0772 */     }
/* FP:mod.rs-0773 */ }
/* FP:mod.rs-0774 */ 
/* FP:mod.rs-0775 */ const TAG_CLEAR_CROSS_CRATE_CLEAR: u8 = 0;
/* FP:mod.rs-0776 */ const TAG_CLEAR_CROSS_CRATE_SET: u8 = 1;
/* FP:mod.rs-0777 */ 
/* FP:mod.rs-0778 */ impl<'tcx, E: TyEncoder<'tcx>, T: Encodable<E>> Encodable<E> for ClearCrossCrate<T> {
/* FP:mod.rs-0779 */     #[inline]
/* FP:mod.rs-0780 */     fn encode(&self, e: &mut E) {
/* FP:mod.rs-0781 */         if E::CLEAR_CROSS_CRATE {
/* FP:mod.rs-0782 */             return;
/* FP:mod.rs-0783 */         }
/* FP:mod.rs-0784 */ 
/* FP:mod.rs-0785 */         match *self {
/* FP:mod.rs-0786 */             ClearCrossCrate::Clear => TAG_CLEAR_CROSS_CRATE_CLEAR.encode(e),
/* FP:mod.rs-0787 */             ClearCrossCrate::Set(ref val) => {
/* FP:mod.rs-0788 */                 TAG_CLEAR_CROSS_CRATE_SET.encode(e);
/* FP:mod.rs-0789 */                 val.encode(e);
/* FP:mod.rs-0790 */             }
/* FP:mod.rs-0791 */         }
/* FP:mod.rs-0792 */     }
/* FP:mod.rs-0793 */ }
/* FP:mod.rs-0794 */ impl<'tcx, D: TyDecoder<'tcx>, T: Decodable<D>> Decodable<D> for ClearCrossCrate<T> {
/* FP:mod.rs-0795 */     #[inline]
/* FP:mod.rs-0796 */     fn decode(d: &mut D) -> ClearCrossCrate<T> {
/* FP:mod.rs-0797 */         if D::CLEAR_CROSS_CRATE {
/* FP:mod.rs-0798 */             return ClearCrossCrate::Clear;
/* FP:mod.rs-0799 */         }
/* FP:mod.rs-0800 */ 
/* FP:mod.rs-0801 */         let discr = u8::decode(d);
/* FP:mod.rs-0802 */ 
/* FP:mod.rs-0803 */         match discr {
/* FP:mod.rs-0804 */             TAG_CLEAR_CROSS_CRATE_CLEAR => ClearCrossCrate::Clear,
/* FP:mod.rs-0805 */             TAG_CLEAR_CROSS_CRATE_SET => {
/* FP:mod.rs-0806 */                 let val = T::decode(d);
/* FP:mod.rs-0807 */                 ClearCrossCrate::Set(val)
/* FP:mod.rs-0808 */             }
/* FP:mod.rs-0809 */             tag => panic!("Invalid tag for ClearCrossCrate: {tag:?}"),
/* FP:mod.rs-0810 */         }
/* FP:mod.rs-0811 */     }
/* FP:mod.rs-0812 */ }
/* FP:mod.rs-0813 */ 
/* FP:mod.rs-0814 */ /// Grouped information about the source code origin of a MIR entity.
/* FP:mod.rs-0815 */ /// Intended to be inspected by diagnostics and debuginfo.
/* FP:mod.rs-0816 */ /// Most passes can work with it as a whole, within a single function.
/* FP:mod.rs-0817 */ // The unofficial Cranelift backend, at least as of #65828, needs `SourceInfo` to implement `Eq` and
/* FP:mod.rs-0818 */ // `Hash`. Please ping @bjorn3 if removing them.
/* FP:mod.rs-0819 */ #[derive(Copy, Clone, Debug, Eq, PartialEq, TyEncodable, TyDecodable, Hash, HashStable)]
/* FP:mod.rs-0820 */ pub struct SourceInfo {
/* FP:mod.rs-0821 */     /// The source span for the AST pertaining to this MIR entity.
/* FP:mod.rs-0822 */     pub span: Span,
/* FP:mod.rs-0823 */ 
/* FP:mod.rs-0824 */     /// The source scope, keeping track of which bindings can be
/* FP:mod.rs-0825 */     /// seen by debuginfo, active lint levels, etc.
/* FP:mod.rs-0826 */     pub scope: SourceScope,
/* FP:mod.rs-0827 */ }
/* FP:mod.rs-0828 */ 
/* FP:mod.rs-0829 */ impl SourceInfo {
/* FP:mod.rs-0830 */     #[inline]
/* FP:mod.rs-0831 */     pub fn outermost(span: Span) -> Self {
/* FP:mod.rs-0832 */         SourceInfo { span, scope: OUTERMOST_SOURCE_SCOPE }
/* FP:mod.rs-0833 */     }
/* FP:mod.rs-0834 */ }
/* FP:mod.rs-0835 */ 
/* FP:mod.rs-0836 */ ///////////////////////////////////////////////////////////////////////////
/* FP:mod.rs-0837 */ // Variables and temps
/* FP:mod.rs-0838 */ 
/* FP:mod.rs-0839 */ crate::rustc_index::newtype_index! {
/* FP:mod.rs-0840 */     #[derive(HashStable)]
/* FP:mod.rs-0841 */     #[encodable]
/* FP:mod.rs-0842 */     #[orderable]
/* FP:mod.rs-0843 */     #[debug_format = "_{}"]
/* FP:mod.rs-0844 */     pub struct Local {
/* FP:mod.rs-0845 */         const RETURN_PLACE = 0;
/* FP:mod.rs-0846 */     }
/* FP:mod.rs-0847 */ }
/* FP:mod.rs-0848 */ 
/* FP:mod.rs-0849 */ impl Atom for Local {
/* FP:mod.rs-0850 */     fn index(self) -> usize {
/* FP:mod.rs-0851 */         Idx::index(self)
/* FP:mod.rs-0852 */     }
/* FP:mod.rs-0853 */ }
/* FP:mod.rs-0854 */ 
/* FP:mod.rs-0855 */ /// Classifies locals into categories. See `Body::local_kind`.
/* FP:mod.rs-0856 */ #[derive(Clone, Copy, PartialEq, Eq, Debug, HashStable)]
/* FP:mod.rs-0857 */ pub enum LocalKind {
/* FP:mod.rs-0858 */     /// User-declared variable binding or compiler-introduced temporary.
/* FP:mod.rs-0859 */     Temp,
/* FP:mod.rs-0860 */     /// Function argument.
/* FP:mod.rs-0861 */     Arg,
/* FP:mod.rs-0862 */     /// Location of function's return value.
/* FP:mod.rs-0863 */     ReturnPointer,
/* FP:mod.rs-0864 */ }
/* FP:mod.rs-0865 */ 
/* FP:mod.rs-0866 */ #[derive(Clone, Debug, TyEncodable, TyDecodable, HashStable)]
/* FP:mod.rs-0867 */ pub struct VarBindingForm<'tcx> {
/* FP:mod.rs-0868 */     /// Is variable bound via `x`, `mut x`, `ref x`, `ref mut x`, `mut ref x`, or `mut ref mut x`?
/* FP:mod.rs-0869 */     pub binding_mode: BindingMode,
/* FP:mod.rs-0870 */     /// If an explicit type was provided for this variable binding,
/* FP:mod.rs-0871 */     /// this holds the source Span of that type.
/* FP:mod.rs-0872 */     ///
/* FP:mod.rs-0873 */     /// NOTE: if you want to change this to a `HirId`, be wary that
/* FP:mod.rs-0874 */     /// doing so breaks incremental compilation (as of this writing),
/* FP:mod.rs-0875 */     /// while a `Span` does not cause our tests to fail.
/* FP:mod.rs-0876 */     pub opt_ty_info: Option<Span>,
/* FP:mod.rs-0877 */     /// Place of the RHS of the =, or the subject of the `match` where this
/* FP:mod.rs-0878 */     /// variable is initialized. None in the case of `let PATTERN;`.
/* FP:mod.rs-0879 */     /// Some((None, ..)) in the case of and `let [mut] x = ...` because
/* FP:mod.rs-0880 */     /// (a) the right-hand side isn't evaluated as a place expression.
/* FP:mod.rs-0881 */     /// (b) it gives a way to separate this case from the remaining cases
/* FP:mod.rs-0882 */     ///     for diagnostics.
/* FP:mod.rs-0883 */     pub opt_match_place: Option<(Option<Place<'tcx>>, Span)>,
/* FP:mod.rs-0884 */     /// The span of the pattern in which this variable was bound.
/* FP:mod.rs-0885 */     pub pat_span: Span,
/* FP:mod.rs-0886 */ }
/* FP:mod.rs-0887 */ 
/* FP:mod.rs-0888 */ #[derive(Clone, Debug, TyEncodable, TyDecodable)]
/* FP:mod.rs-0889 */ pub enum BindingForm<'tcx> {
/* FP:mod.rs-0890 */     /// This is a binding for a non-`self` binding, or a `self` that has an explicit type.
/* FP:mod.rs-0891 */     Var(VarBindingForm<'tcx>),
/* FP:mod.rs-0892 */     /// Binding for a `self`/`&self`/`&mut self` binding where the type is implicit.
/* FP:mod.rs-0893 */     ImplicitSelf(ImplicitSelfKind),
/* FP:mod.rs-0894 */     /// Reference used in a guard expression to ensure immutability.
/* FP:mod.rs-0895 */     RefForGuard,
/* FP:mod.rs-0896 */ }
/* FP:mod.rs-0897 */ 
/* FP:mod.rs-0898 */ mod binding_form_impl {
/* FP:mod.rs-0899 */     use crate::rustc_data_structures::stable_hasher::{HashStable, StableHasher};
/* FP:mod.rs-0900 */     use rustc_query_system::ich::StableHashingContext;
/* FP:mod.rs-0901 */ 
/* FP:mod.rs-0902 */     impl<'a, 'tcx> HashStable<StableHashingContext<'a>> for super::BindingForm<'tcx> {
/* FP:mod.rs-0903 */         fn hash_stable(&self, hcx: &mut StableHashingContext<'a>, hasher: &mut StableHasher) {
/* FP:mod.rs-0904 */             use super::BindingForm::*;
/* FP:mod.rs-0905 */             std::mem::discriminant(self).hash_stable(hcx, hasher);
/* FP:mod.rs-0906 */ 
/* FP:mod.rs-0907 */             match self {
/* FP:mod.rs-0908 */                 Var(binding) => binding.hash_stable(hcx, hasher),
/* FP:mod.rs-0909 */                 ImplicitSelf(kind) => kind.hash_stable(hcx, hasher),
/* FP:mod.rs-0910 */                 RefForGuard => (),
/* FP:mod.rs-0911 */             }
/* FP:mod.rs-0912 */         }
/* FP:mod.rs-0913 */     }
/* FP:mod.rs-0914 */ }
/* FP:mod.rs-0915 */ 
/* FP:mod.rs-0916 */ /// `BlockTailInfo` is attached to the `LocalDecl` for temporaries
/* FP:mod.rs-0917 */ /// created during evaluation of expressions in a block tail
/* FP:mod.rs-0918 */ /// expression; that is, a block like `{ STMT_1; STMT_2; EXPR }`.
/* FP:mod.rs-0919 */ ///
/* FP:mod.rs-0920 */ /// It is used to improve diagnostics when such temporaries are
/* FP:mod.rs-0921 */ /// involved in borrow_check errors, e.g., explanations of where the
/* FP:mod.rs-0922 */ /// temporaries come from, when their destructors are run, and/or how
/* FP:mod.rs-0923 */ /// one might revise the code to satisfy the borrow checker's rules.
/* FP:mod.rs-0924 */ #[derive(Clone, Copy, Debug, PartialEq, Eq, TyEncodable, TyDecodable, HashStable)]
/* FP:mod.rs-0925 */ pub struct BlockTailInfo {
/* FP:mod.rs-0926 */     /// If `true`, then the value resulting from evaluating this tail
/* FP:mod.rs-0927 */     /// expression is ignored by the block's expression context.
/* FP:mod.rs-0928 */     ///
/* FP:mod.rs-0929 */     /// Examples include `{ ...; tail };` and `let _ = { ...; tail };`
/* FP:mod.rs-0930 */     /// but not e.g., `let _x = { ...; tail };`
/* FP:mod.rs-0931 */     pub tail_result_is_ignored: bool,
/* FP:mod.rs-0932 */ 
/* FP:mod.rs-0933 */     /// `Span` of the tail expression.
/* FP:mod.rs-0934 */     pub span: Span,
/* FP:mod.rs-0935 */ }
/* FP:mod.rs-0936 */ 
/* FP:mod.rs-0937 */ /// A MIR local.
/* FP:mod.rs-0938 */ ///
/* FP:mod.rs-0939 */ /// This can be a binding declared by the user, a temporary inserted by the compiler, a function
/* FP:mod.rs-0940 */ /// argument, or the return place.
/* FP:mod.rs-0941 */ #[derive(Clone, Debug, TyEncodable, TyDecodable, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-0942 */ pub struct LocalDecl<'tcx> {
/* FP:mod.rs-0943 */     /// Whether this is a mutable binding (i.e., `let x` or `let mut x`).
/* FP:mod.rs-0944 */     ///
/* FP:mod.rs-0945 */     /// Temporaries and the return place are always mutable.
/* FP:mod.rs-0946 */     pub mutability: Mutability,
/* FP:mod.rs-0947 */ 
/* FP:mod.rs-0948 */     pub local_info: ClearCrossCrate<Box<LocalInfo<'tcx>>>,
/* FP:mod.rs-0949 */ 
/* FP:mod.rs-0950 */     /// The type of this local.
/* FP:mod.rs-0951 */     pub ty: Ty<'tcx>,
/* FP:mod.rs-0952 */ 
/* FP:mod.rs-0953 */     /// If the user manually ascribed a type to this variable,
/* FP:mod.rs-0954 */     /// e.g., via `let x: T`, then we carry that type here. The MIR
/* FP:mod.rs-0955 */     /// borrow checker needs this information since it can affect
/* FP:mod.rs-0956 */     /// region inference.
/* FP:mod.rs-0957 */     pub user_ty: Option<Box<UserTypeProjections>>,
/* FP:mod.rs-0958 */ 
/* FP:mod.rs-0959 */     /// The *syntactic* (i.e., not visibility) source scope the local is defined
/* FP:mod.rs-0960 */     /// in. If the local was defined in a let-statement, this
/* FP:mod.rs-0961 */     /// is *within* the let-statement, rather than outside
/* FP:mod.rs-0962 */     /// of it.
/* FP:mod.rs-0963 */     ///
/* FP:mod.rs-0964 */     /// This is needed because the visibility source scope of locals within
/* FP:mod.rs-0965 */     /// a let-statement is weird.
/* FP:mod.rs-0966 */     ///
/* FP:mod.rs-0967 */     /// The reason is that we want the local to be *within* the let-statement
/* FP:mod.rs-0968 */     /// for lint purposes, but we want the local to be *after* the let-statement
/* FP:mod.rs-0969 */     /// for names-in-scope purposes.
/* FP:mod.rs-0970 */     ///
/* FP:mod.rs-0971 */     /// That's it, if we have a let-statement like the one in this
/* FP:mod.rs-0972 */     /// function:
/* FP:mod.rs-0973 */     ///
/* FP:mod.rs-0974 */     /// ```
/* FP:mod.rs-0975 */     /// fn foo(x: &str) {
/* FP:mod.rs-0976 */     ///     #[allow(unused_mut)]
/* FP:mod.rs-0977 */     ///     let mut x: u32 = {
/* FP:mod.rs-0978 */     ///         //^ one unused mut
/* FP:mod.rs-0979 */     ///         let mut y: u32 = x.parse().unwrap();
/* FP:mod.rs-0980 */     ///         y + 2
/* FP:mod.rs-0981 */     ///     };
/* FP:mod.rs-0982 */     ///     drop(x);
/* FP:mod.rs-0983 */     /// }
/* FP:mod.rs-0984 */     /// ```
/* FP:mod.rs-0985 */     ///
/* FP:mod.rs-0986 */     /// Then, from a lint point of view, the declaration of `x: u32`
/* FP:mod.rs-0987 */     /// (and `y: u32`) are within the `#[allow(unused_mut)]` scope - the
/* FP:mod.rs-0988 */     /// lint scopes are the same as the AST/HIR nesting.
/* FP:mod.rs-0989 */     ///
/* FP:mod.rs-0990 */     /// However, from a name lookup point of view, the scopes look more like
/* FP:mod.rs-0991 */     /// as if the let-statements were `match` expressions:
/* FP:mod.rs-0992 */     ///
/* FP:mod.rs-0993 */     /// ```
/* FP:mod.rs-0994 */     /// fn foo(x: &str) {
/* FP:mod.rs-0995 */     ///     match {
/* FP:mod.rs-0996 */     ///         match x.parse::<u32>().unwrap() {
/* FP:mod.rs-0997 */     ///             y => y + 2
/* FP:mod.rs-0998 */     ///         }
/* FP:mod.rs-0999 */     ///     } {
/* FP:mod.rs-1000 */     ///         x => drop(x)
/* FP:mod.rs-1001 */     ///     };
/* FP:mod.rs-1002 */     /// }
/* FP:mod.rs-1003 */     /// ```
/* FP:mod.rs-1004 */     ///
/* FP:mod.rs-1005 */     /// We care about the name-lookup scopes for debuginfo - if the
/* FP:mod.rs-1006 */     /// debuginfo instruction pointer is at the call to `x.parse()`, we
/* FP:mod.rs-1007 */     /// want `x` to refer to `x: &str`, but if it is at the call to
/* FP:mod.rs-1008 */     /// `drop(x)`, we want it to refer to `x: u32`.
/* FP:mod.rs-1009 */     ///
/* FP:mod.rs-1010 */     /// To allow both uses to work, we need to have more than a single scope
/* FP:mod.rs-1011 */     /// for a local. We have the `source_info.scope` represent the "syntactic"
/* FP:mod.rs-1012 */     /// lint scope (with a variable being under its let block) while the
/* FP:mod.rs-1013 */     /// `var_debug_info.source_info.scope` represents the "local variable"
/* FP:mod.rs-1014 */     /// scope (where the "rest" of a block is under all prior let-statements).
/* FP:mod.rs-1015 */     ///
/* FP:mod.rs-1016 */     /// The end result looks like this:
/* FP:mod.rs-1017 */     ///
/* FP:mod.rs-1018 */     /// ```text
/* FP:mod.rs-1019 */     /// ROOT SCOPE
/* FP:mod.rs-1020 */     ///  │{ argument x: &str }
/* FP:mod.rs-1021 */     ///  │
/* FP:mod.rs-1022 */     ///  │ │{ #[allow(unused_mut)] } // This is actually split into 2 scopes
/* FP:mod.rs-1023 */     ///  │ │                         // in practice because I'm lazy.
/* FP:mod.rs-1024 */     ///  │ │
/* FP:mod.rs-1025 */     ///  │ │← x.source_info.scope
/* FP:mod.rs-1026 */     ///  │ │← `x.parse().unwrap()`
/* FP:mod.rs-1027 */     ///  │ │
/* FP:mod.rs-1028 */     ///  │ │ │← y.source_info.scope
/* FP:mod.rs-1029 */     ///  │ │
/* FP:mod.rs-1030 */     ///  │ │ │{ let y: u32 }
/* FP:mod.rs-1031 */     ///  │ │ │
/* FP:mod.rs-1032 */     ///  │ │ │← y.var_debug_info.source_info.scope
/* FP:mod.rs-1033 */     ///  │ │ │← `y + 2`
/* FP:mod.rs-1034 */     ///  │
/* FP:mod.rs-1035 */     ///  │ │{ let x: u32 }
/* FP:mod.rs-1036 */     ///  │ │← x.var_debug_info.source_info.scope
/* FP:mod.rs-1037 */     ///  │ │← `drop(x)` // This accesses `x: u32`.
/* FP:mod.rs-1038 */     /// ```
/* FP:mod.rs-1039 */     pub source_info: SourceInfo,
/* FP:mod.rs-1040 */ }
/* FP:mod.rs-1041 */ 
/* FP:mod.rs-1042 */ /// Extra information about a some locals that's used for diagnostics and for
/* FP:mod.rs-1043 */ /// classifying variables into local variables, statics, etc, which is needed e.g.
/* FP:mod.rs-1044 */ /// for borrow checking.
/* FP:mod.rs-1045 */ ///
/* FP:mod.rs-1046 */ /// Not used for non-StaticRef temporaries, the return place, or anonymous
/* FP:mod.rs-1047 */ /// function parameters.
/* FP:mod.rs-1048 */ #[derive(Clone, Debug, TyEncodable, TyDecodable, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-1049 */ pub enum LocalInfo<'tcx> {
/* FP:mod.rs-1050 */     /// A user-defined local variable or function parameter
/* FP:mod.rs-1051 */     ///
/* FP:mod.rs-1052 */     /// The `BindingForm` is solely used for local diagnostics when generating
/* FP:mod.rs-1053 */     /// warnings/errors when compiling the current crate, and therefore it need
/* FP:mod.rs-1054 */     /// not be visible across crates.
/* FP:mod.rs-1055 */     User(BindingForm<'tcx>),
/* FP:mod.rs-1056 */     /// A temporary created that references the static with the given `DefId`.
/* FP:mod.rs-1057 */     StaticRef { def_id: DefId, is_thread_local: bool },
/* FP:mod.rs-1058 */     /// A temporary created that references the const with the given `DefId`
/* FP:mod.rs-1059 */     ConstRef { def_id: DefId },
/* FP:mod.rs-1060 */     /// A temporary created during the creation of an aggregate
/* FP:mod.rs-1061 */     /// (e.g. a temporary for `foo` in `MyStruct { my_field: foo }`)
/* FP:mod.rs-1062 */     AggregateTemp,
/* FP:mod.rs-1063 */     /// A temporary created for evaluation of some subexpression of some block's tail expression
/* FP:mod.rs-1064 */     /// (with no intervening statement context).
/* FP:mod.rs-1065 */     BlockTailTemp(BlockTailInfo),
/* FP:mod.rs-1066 */     /// A temporary created during evaluating `if` predicate, possibly for pattern matching for `let`s,
/* FP:mod.rs-1067 */     /// and subject to Edition 2024 temporary lifetime rules
/* FP:mod.rs-1068 */     IfThenRescopeTemp { if_then: HirId },
/* FP:mod.rs-1069 */     /// A temporary created during the pass `Derefer` to avoid it's retagging
/* FP:mod.rs-1070 */     DerefTemp,
/* FP:mod.rs-1071 */     /// A temporary created for borrow checking.
/* FP:mod.rs-1072 */     FakeBorrow,
/* FP:mod.rs-1073 */     /// A local without anything interesting about it.
/* FP:mod.rs-1074 */     Boring,
/* FP:mod.rs-1075 */ }
/* FP:mod.rs-1076 */ 
/* FP:mod.rs-1077 */ impl<'tcx> LocalDecl<'tcx> {
/* FP:mod.rs-1078 */     pub fn local_info(&self) -> &LocalInfo<'tcx> {
/* FP:mod.rs-1079 */         self.local_info.as_ref().unwrap_crate_local()
/* FP:mod.rs-1080 */     }
/* FP:mod.rs-1081 */ 
/* FP:mod.rs-1082 */     /// Returns `true` only if local is a binding that can itself be
/* FP:mod.rs-1083 */     /// made mutable via the addition of the `mut` keyword, namely
/* FP:mod.rs-1084 */     /// something like the occurrences of `x` in:
/* FP:mod.rs-1085 */     /// - `fn foo(x: Type) { ... }`,
/* FP:mod.rs-1086 */     /// - `let x = ...`,
/* FP:mod.rs-1087 */     /// - or `match ... { C(x) => ... }`
/* FP:mod.rs-1088 */     pub fn can_be_made_mutable(&self) -> bool {
/* FP:mod.rs-1089 */         matches!(
/* FP:mod.rs-1090 */             self.local_info(),
/* FP:mod.rs-1091 */             LocalInfo::User(
/* FP:mod.rs-1092 */                 BindingForm::Var(VarBindingForm {
/* FP:mod.rs-1093 */                     binding_mode: BindingMode(ByRef::No, _),
/* FP:mod.rs-1094 */                     opt_ty_info: _,
/* FP:mod.rs-1095 */                     opt_match_place: _,
/* FP:mod.rs-1096 */                     pat_span: _,
/* FP:mod.rs-1097 */                 }) | BindingForm::ImplicitSelf(ImplicitSelfKind::Imm),
/* FP:mod.rs-1098 */             )
/* FP:mod.rs-1099 */         )
/* FP:mod.rs-1100 */     }
/* FP:mod.rs-1101 */ 
/* FP:mod.rs-1102 */     /// Returns `true` if local is definitely not a `ref ident` or
/* FP:mod.rs-1103 */     /// `ref mut ident` binding. (Such bindings cannot be made into
/* FP:mod.rs-1104 */     /// mutable bindings, but the inverse does not necessarily hold).
/* FP:mod.rs-1105 */     pub fn is_nonref_binding(&self) -> bool {
/* FP:mod.rs-1106 */         matches!(
/* FP:mod.rs-1107 */             self.local_info(),
/* FP:mod.rs-1108 */             LocalInfo::User(
/* FP:mod.rs-1109 */                 BindingForm::Var(VarBindingForm {
/* FP:mod.rs-1110 */                     binding_mode: BindingMode(ByRef::No, _),
/* FP:mod.rs-1111 */                     opt_ty_info: _,
/* FP:mod.rs-1112 */                     opt_match_place: _,
/* FP:mod.rs-1113 */                     pat_span: _,
/* FP:mod.rs-1114 */                 }) | BindingForm::ImplicitSelf(_),
/* FP:mod.rs-1115 */             )
/* FP:mod.rs-1116 */         )
/* FP:mod.rs-1117 */     }
/* FP:mod.rs-1118 */ 
/* FP:mod.rs-1119 */     /// Returns `true` if this variable is a named variable or function
/* FP:mod.rs-1120 */     /// parameter declared by the user.
/* FP:mod.rs-1121 */     #[inline]
/* FP:mod.rs-1122 */     pub fn is_user_variable(&self) -> bool {
/* FP:mod.rs-1123 */         matches!(self.local_info(), LocalInfo::User(_))
/* FP:mod.rs-1124 */     }
/* FP:mod.rs-1125 */ 
/* FP:mod.rs-1126 */     /// Returns `true` if this is a reference to a variable bound in a `match`
/* FP:mod.rs-1127 */     /// expression that is used to access said variable for the guard of the
/* FP:mod.rs-1128 */     /// match arm.
/* FP:mod.rs-1129 */     pub fn is_ref_for_guard(&self) -> bool {
/* FP:mod.rs-1130 */         matches!(self.local_info(), LocalInfo::User(BindingForm::RefForGuard))
/* FP:mod.rs-1131 */     }
/* FP:mod.rs-1132 */ 
/* FP:mod.rs-1133 */     /// Returns `Some` if this is a reference to a static item that is used to
/* FP:mod.rs-1134 */     /// access that static.
/* FP:mod.rs-1135 */     pub fn is_ref_to_static(&self) -> bool {
/* FP:mod.rs-1136 */         matches!(self.local_info(), LocalInfo::StaticRef { .. })
/* FP:mod.rs-1137 */     }
/* FP:mod.rs-1138 */ 
/* FP:mod.rs-1139 */     /// Returns `Some` if this is a reference to a thread-local static item that is used to
/* FP:mod.rs-1140 */     /// access that static.
/* FP:mod.rs-1141 */     pub fn is_ref_to_thread_local(&self) -> bool {
/* FP:mod.rs-1142 */         match self.local_info() {
/* FP:mod.rs-1143 */             LocalInfo::StaticRef { is_thread_local, .. } => *is_thread_local,
/* FP:mod.rs-1144 */             _ => false,
/* FP:mod.rs-1145 */         }
/* FP:mod.rs-1146 */     }
/* FP:mod.rs-1147 */ 
/* FP:mod.rs-1148 */     /// Returns `true` if this is a DerefTemp
/* FP:mod.rs-1149 */     pub fn is_deref_temp(&self) -> bool {
/* FP:mod.rs-1150 */         match self.local_info() {
/* FP:mod.rs-1151 */             LocalInfo::DerefTemp => true,
/* FP:mod.rs-1152 */             _ => false,
/* FP:mod.rs-1153 */         }
/* FP:mod.rs-1154 */     }
/* FP:mod.rs-1155 */ 
/* FP:mod.rs-1156 */     /// Returns `true` is the local is from a compiler desugaring, e.g.,
/* FP:mod.rs-1157 */     /// `__next` from a `for` loop.
/* FP:mod.rs-1158 */     #[inline]
/* FP:mod.rs-1159 */     pub fn from_compiler_desugaring(&self) -> bool {
/* FP:mod.rs-1160 */         self.source_info.span.desugaring_kind().is_some()
/* FP:mod.rs-1161 */     }
/* FP:mod.rs-1162 */ 
/* FP:mod.rs-1163 */     /// Creates a new `LocalDecl` for a temporary, mutable.
/* FP:mod.rs-1164 */     #[inline]
/* FP:mod.rs-1165 */     pub fn new(ty: Ty<'tcx>, span: Span) -> Self {
/* FP:mod.rs-1166 */         Self::with_source_info(ty, SourceInfo::outermost(span))
/* FP:mod.rs-1167 */     }
/* FP:mod.rs-1168 */ 
/* FP:mod.rs-1169 */     /// Like `LocalDecl::new`, but takes a `SourceInfo` instead of a `Span`.
/* FP:mod.rs-1170 */     #[inline]
/* FP:mod.rs-1171 */     pub fn with_source_info(ty: Ty<'tcx>, source_info: SourceInfo) -> Self {
/* FP:mod.rs-1172 */         LocalDecl {
/* FP:mod.rs-1173 */             mutability: Mutability::Mut,
/* FP:mod.rs-1174 */             local_info: ClearCrossCrate::Set(Box::new(LocalInfo::Boring)),
/* FP:mod.rs-1175 */             ty,
/* FP:mod.rs-1176 */             user_ty: None,
/* FP:mod.rs-1177 */             source_info,
/* FP:mod.rs-1178 */         }
/* FP:mod.rs-1179 */     }
/* FP:mod.rs-1180 */ 
/* FP:mod.rs-1181 */     /// Converts `self` into same `LocalDecl` except tagged as immutable.
/* FP:mod.rs-1182 */     #[inline]
/* FP:mod.rs-1183 */     pub fn immutable(mut self) -> Self {
/* FP:mod.rs-1184 */         self.mutability = Mutability::Not;
/* FP:mod.rs-1185 */         self
/* FP:mod.rs-1186 */     }
/* FP:mod.rs-1187 */ }
/* FP:mod.rs-1188 */ 
/* FP:mod.rs-1189 */ #[derive(Clone, TyEncodable, TyDecodable, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-1190 */ pub enum VarDebugInfoContents<'tcx> {
/* FP:mod.rs-1191 */     /// This `Place` only contains projection which satisfy `can_use_in_debuginfo`.
/* FP:mod.rs-1192 */     Place(Place<'tcx>),
/* FP:mod.rs-1193 */     Const(ConstOperand<'tcx>),
/* FP:mod.rs-1194 */ }
/* FP:mod.rs-1195 */ 
/* FP:mod.rs-1196 */ impl<'tcx> Debug for VarDebugInfoContents<'tcx> {
/* FP:mod.rs-1197 */     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
/* FP:mod.rs-1198 */         match self {
/* FP:mod.rs-1199 */             VarDebugInfoContents::Const(c) => write!(fmt, "{c}"),
/* FP:mod.rs-1200 */             VarDebugInfoContents::Place(p) => write!(fmt, "{p:?}"),
/* FP:mod.rs-1201 */         }
/* FP:mod.rs-1202 */     }
/* FP:mod.rs-1203 */ }
/* FP:mod.rs-1204 */ 
/* FP:mod.rs-1205 */ #[derive(Clone, Debug, TyEncodable, TyDecodable, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-1206 */ pub struct VarDebugInfoFragment<'tcx> {
/* FP:mod.rs-1207 */     /// Type of the original user variable.
/* FP:mod.rs-1208 */     /// This cannot contain a union or an enum.
/* FP:mod.rs-1209 */     pub ty: Ty<'tcx>,
/* FP:mod.rs-1210 */ 
/* FP:mod.rs-1211 */     /// Where in the composite user variable this fragment is,
/* FP:mod.rs-1212 */     /// represented as a "projection" into the composite variable.
/* FP:mod.rs-1213 */     /// At lower levels, this corresponds to a byte/bit range.
/* FP:mod.rs-1214 */     ///
/* FP:mod.rs-1215 */     /// This can only contain `PlaceElem::Field`.
/* FP:mod.rs-1216 */     // FIXME support this for `enum`s by either using DWARF's
/* FP:mod.rs-1217 */     // more advanced control-flow features (unsupported by LLVM?)
/* FP:mod.rs-1218 */     // to match on the discriminant, or by using custom type debuginfo
/* FP:mod.rs-1219 */     // with non-overlapping variants for the composite variable.
/* FP:mod.rs-1220 */     pub projection: Vec<PlaceElem<'tcx>>,
/* FP:mod.rs-1221 */ }
/* FP:mod.rs-1222 */ 
/* FP:mod.rs-1223 */ /// Debug information pertaining to a user variable.
/* FP:mod.rs-1224 */ #[derive(Clone, TyEncodable, TyDecodable, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-1225 */ pub struct VarDebugInfo<'tcx> {
/* FP:mod.rs-1226 */     pub name: Symbol,
/* FP:mod.rs-1227 */ 
/* FP:mod.rs-1228 */     /// Source info of the user variable, including the scope
/* FP:mod.rs-1229 */     /// within which the variable is visible (to debuginfo)
/* FP:mod.rs-1230 */     /// (see `LocalDecl`'s `source_info` field for more details).
/* FP:mod.rs-1231 */     pub source_info: SourceInfo,
/* FP:mod.rs-1232 */ 
/* FP:mod.rs-1233 */     /// The user variable's data is split across several fragments,
/* FP:mod.rs-1234 */     /// each described by a `VarDebugInfoFragment`.
/* FP:mod.rs-1235 */     /// See DWARF 5's "2.6.1.2 Composite Location Descriptions"
/* FP:mod.rs-1236 */     /// and LLVM's `DW_OP_LLVM_fragment` for more details on
/* FP:mod.rs-1237 */     /// the underlying debuginfo feature this relies on.
/* FP:mod.rs-1238 */     pub composite: Option<Box<VarDebugInfoFragment<'tcx>>>,
/* FP:mod.rs-1239 */ 
/* FP:mod.rs-1240 */     /// Where the data for this user variable is to be found.
/* FP:mod.rs-1241 */     pub value: VarDebugInfoContents<'tcx>,
/* FP:mod.rs-1242 */ 
/* FP:mod.rs-1243 */     /// When present, indicates what argument number this variable is in the function that it
/* FP:mod.rs-1244 */     /// originated from (starting from 1). Note, if MIR inlining is enabled, then this is the
/* FP:mod.rs-1245 */     /// argument number in the original function before it was inlined.
/* FP:mod.rs-1246 */     pub argument_index: Option<u16>,
/* FP:mod.rs-1247 */ }
/* FP:mod.rs-1248 */ 
/* FP:mod.rs-1249 */ ///////////////////////////////////////////////////////////////////////////
/* FP:mod.rs-1250 */ // BasicBlock
/* FP:mod.rs-1251 */ 
/* FP:mod.rs-1252 */ crate::rustc_index::newtype_index! {
/* FP:mod.rs-1253 */     /// A node in the MIR [control-flow graph][CFG].
/* FP:mod.rs-1254 */     ///
/* FP:mod.rs-1255 */     /// There are no branches (e.g., `if`s, function calls, etc.) within a basic block, which makes
/* FP:mod.rs-1256 */     /// it easier to do [data-flow analyses] and optimizations. Instead, branches are represented
/* FP:mod.rs-1257 */     /// as an edge in a graph between basic blocks.
/* FP:mod.rs-1258 */     ///
/* FP:mod.rs-1259 */     /// Basic blocks consist of a series of [statements][Statement], ending with a
/* FP:mod.rs-1260 */     /// [terminator][Terminator]. Basic blocks can have multiple predecessors and successors,
/* FP:mod.rs-1261 */     /// however there is a MIR pass ([`CriticalCallEdges`]) that removes *critical edges*, which
/* FP:mod.rs-1262 */     /// are edges that go from a multi-successor node to a multi-predecessor node. This pass is
/* FP:mod.rs-1263 */     /// needed because some analyses require that there are no critical edges in the CFG.
/* FP:mod.rs-1264 */     ///
/* FP:mod.rs-1265 */     /// Note that this type is just an index into [`Body.basic_blocks`](Body::basic_blocks);
/* FP:mod.rs-1266 */     /// the actual data that a basic block holds is in [`BasicBlockData`].
/* FP:mod.rs-1267 */     ///
/* FP:mod.rs-1268 */     /// Read more about basic blocks in the [rustc-dev-guide][guide-mir].
/* FP:mod.rs-1269 */     ///
/* FP:mod.rs-1270 */     /// [CFG]: https://rustc-dev-guide.rust-lang.org/appendix/background.html#cfg
/* FP:mod.rs-1271 */     /// [data-flow analyses]:
/* FP:mod.rs-1272 */     ///     https://rustc-dev-guide.rust-lang.org/appendix/background.html#what-is-a-dataflow-analysis
/* FP:mod.rs-1273 */     /// [`CriticalCallEdges`]: ../../rustc_mir_transform/add_call_guards/enum.AddCallGuards.html#variant.CriticalCallEdges
/* FP:mod.rs-1274 */     /// [guide-mir]: https://rustc-dev-guide.rust-lang.org/mir/
/* FP:mod.rs-1275 */     #[derive(HashStable)]
/* FP:mod.rs-1276 */     #[encodable]
/* FP:mod.rs-1277 */     #[orderable]
/* FP:mod.rs-1278 */     #[debug_format = "bb{}"]
/* FP:mod.rs-1279 */     pub struct BasicBlock {
/* FP:mod.rs-1280 */         const START_BLOCK = 0;
/* FP:mod.rs-1281 */     }
/* FP:mod.rs-1282 */ }
/* FP:mod.rs-1283 */ 
/* FP:mod.rs-1284 */ impl BasicBlock {
/* FP:mod.rs-1285 */     pub fn start_location(self) -> Location {
/* FP:mod.rs-1286 */         Location { block: self, statement_index: 0 }
/* FP:mod.rs-1287 */     }
/* FP:mod.rs-1288 */ }
/* FP:mod.rs-1289 */ 
/* FP:mod.rs-1290 */ ///////////////////////////////////////////////////////////////////////////
/* FP:mod.rs-1291 */ // BasicBlockData
/* FP:mod.rs-1292 */ 
/* FP:mod.rs-1293 */ /// Data for a basic block, including a list of its statements.
/* FP:mod.rs-1294 */ ///
/* FP:mod.rs-1295 */ /// See [`BasicBlock`] for documentation on what basic blocks are at a high level.
/* FP:mod.rs-1296 */ #[derive(Clone, Debug, TyEncodable, TyDecodable, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-1297 */ #[non_exhaustive]
/* FP:mod.rs-1298 */ pub struct BasicBlockData<'tcx> {
/* FP:mod.rs-1299 */     /// List of statements in this block.
/* FP:mod.rs-1300 */     pub statements: Vec<Statement<'tcx>>,
/* FP:mod.rs-1301 */ 
/* FP:mod.rs-1302 */     /// Terminator for this block.
/* FP:mod.rs-1303 */     ///
/* FP:mod.rs-1304 */     /// N.B., this should generally ONLY be `None` during construction.
/* FP:mod.rs-1305 */     /// Therefore, you should generally access it via the
/* FP:mod.rs-1306 */     /// `terminator()` or `terminator_mut()` methods. The only
/* FP:mod.rs-1307 */     /// exception is that certain passes, such as `simplify_cfg`, swap
/* FP:mod.rs-1308 */     /// out the terminator temporarily with `None` while they continue
/* FP:mod.rs-1309 */     /// to recurse over the set of basic blocks.
/* FP:mod.rs-1310 */     pub terminator: Option<Terminator<'tcx>>,
/* FP:mod.rs-1311 */ 
/* FP:mod.rs-1312 */     /// If true, this block lies on an unwind path. This is used
/* FP:mod.rs-1313 */     /// during codegen where distinct kinds of basic blocks may be
/* FP:mod.rs-1314 */     /// generated (particularly for MSVC cleanup). Unwind blocks must
/* FP:mod.rs-1315 */     /// only branch to other unwind blocks.
/* FP:mod.rs-1316 */     pub is_cleanup: bool,
/* FP:mod.rs-1317 */ }
/* FP:mod.rs-1318 */ 
/* FP:mod.rs-1319 */ impl<'tcx> BasicBlockData<'tcx> {
/* FP:mod.rs-1320 */     pub fn new(terminator: Option<Terminator<'tcx>>, is_cleanup: bool) -> BasicBlockData<'tcx> {
/* FP:mod.rs-1321 */         BasicBlockData::new_stmts(Vec::new(), terminator, is_cleanup)
/* FP:mod.rs-1322 */     }
/* FP:mod.rs-1323 */ 
/* FP:mod.rs-1324 */     pub fn new_stmts(
/* FP:mod.rs-1325 */         statements: Vec<Statement<'tcx>>,
/* FP:mod.rs-1326 */         terminator: Option<Terminator<'tcx>>,
/* FP:mod.rs-1327 */         is_cleanup: bool,
/* FP:mod.rs-1328 */     ) -> BasicBlockData<'tcx> {
/* FP:mod.rs-1329 */         BasicBlockData { statements, terminator, is_cleanup }
/* FP:mod.rs-1330 */     }
/* FP:mod.rs-1331 */ 
/* FP:mod.rs-1332 */     /// Accessor for terminator.
/* FP:mod.rs-1333 */     ///
/* FP:mod.rs-1334 */     /// Terminator may not be None after construction of the basic block is complete. This accessor
/* FP:mod.rs-1335 */     /// provides a convenient way to reach the terminator.
/* FP:mod.rs-1336 */     #[inline]
/* FP:mod.rs-1337 */     pub fn terminator(&self) -> &Terminator<'tcx> {
/* FP:mod.rs-1338 */         self.terminator.as_ref().expect("invalid terminator state")
/* FP:mod.rs-1339 */     }
/* FP:mod.rs-1340 */ 
/* FP:mod.rs-1341 */     #[inline]
/* FP:mod.rs-1342 */     pub fn terminator_mut(&mut self) -> &mut Terminator<'tcx> {
/* FP:mod.rs-1343 */         self.terminator.as_mut().expect("invalid terminator state")
/* FP:mod.rs-1344 */     }
/* FP:mod.rs-1345 */ 
/* FP:mod.rs-1346 */     /// Does the block have no statements and an unreachable terminator?
/* FP:mod.rs-1347 */     #[inline]
/* FP:mod.rs-1348 */     pub fn is_empty_unreachable(&self) -> bool {
/* FP:mod.rs-1349 */         self.statements.is_empty() && matches!(self.terminator().kind, TerminatorKind::Unreachable)
/* FP:mod.rs-1350 */     }
/* FP:mod.rs-1351 */ 
/* FP:mod.rs-1352 */     /// Like [`Terminator::successors`] but tries to use information available from the [`Instance`]
/* FP:mod.rs-1353 */     /// to skip successors like the `false` side of an `if const {`.
/* FP:mod.rs-1354 */     ///
/* FP:mod.rs-1355 */     /// This is used to implement [`traversal::mono_reachable`] and
/* FP:mod.rs-1356 */     /// [`traversal::mono_reachable_reverse_postorder`].
/* FP:mod.rs-1357 */     pub fn mono_successors(&self, tcx: TyCtxt<'tcx>, instance: Instance<'tcx>) -> Successors<'_> {
/* FP:mod.rs-1358 */         if let Some((bits, targets)) = Body::try_const_mono_switchint(tcx, instance, self) {
/* FP:mod.rs-1359 */             targets.successors_for_value(bits)
/* FP:mod.rs-1360 */         } else {
/* FP:mod.rs-1361 */             self.terminator().successors()
/* FP:mod.rs-1362 */         }
/* FP:mod.rs-1363 */     }
/* FP:mod.rs-1364 */ }
/* FP:mod.rs-1365 */ 
/* FP:mod.rs-1366 */ ///////////////////////////////////////////////////////////////////////////
/* FP:mod.rs-1367 */ // Scopes
/* FP:mod.rs-1368 */ 
/* FP:mod.rs-1369 */ crate::rustc_index::newtype_index! {
/* FP:mod.rs-1370 */     #[derive(HashStable)]
/* FP:mod.rs-1371 */     #[encodable]
/* FP:mod.rs-1372 */     #[debug_format = "scope[{}]"]
/* FP:mod.rs-1373 */     pub struct SourceScope {
/* FP:mod.rs-1374 */         const OUTERMOST_SOURCE_SCOPE = 0;
/* FP:mod.rs-1375 */     }
/* FP:mod.rs-1376 */ }
/* FP:mod.rs-1377 */ 
/* FP:mod.rs-1378 */ impl SourceScope {
/* FP:mod.rs-1379 */     /// Finds the original HirId this MIR item came from.
/* FP:mod.rs-1380 */     /// This is necessary after MIR optimizations, as otherwise we get a HirId
/* FP:mod.rs-1381 */     /// from the function that was inlined instead of the function call site.
/* FP:mod.rs-1382 */     pub fn lint_root(
/* FP:mod.rs-1383 */         self,
/* FP:mod.rs-1384 */         source_scopes: &IndexSlice<SourceScope, SourceScopeData<'_>>,
/* FP:mod.rs-1385 */     ) -> Option<HirId> {
/* FP:mod.rs-1386 */         let mut data = &source_scopes[self];
/* FP:mod.rs-1387 */         // FIXME(oli-obk): we should be able to just walk the `inlined_parent_scope`, but it
/* FP:mod.rs-1388 */         // does not work as I thought it would. Needs more investigation and documentation.
/* FP:mod.rs-1389 */         while data.inlined.is_some() {
/* FP:mod.rs-1390 */             trace!(?data);
/* FP:mod.rs-1391 */             data = &source_scopes[data.parent_scope.unwrap()];
/* FP:mod.rs-1392 */         }
/* FP:mod.rs-1393 */         trace!(?data);
/* FP:mod.rs-1394 */         match &data.local_data {
/* FP:mod.rs-1395 */             ClearCrossCrate::Set(data) => Some(data.lint_root),
/* FP:mod.rs-1396 */             ClearCrossCrate::Clear => None,
/* FP:mod.rs-1397 */         }
/* FP:mod.rs-1398 */     }
/* FP:mod.rs-1399 */ 
/* FP:mod.rs-1400 */     /// The instance this source scope was inlined from, if any.
/* FP:mod.rs-1401 */     #[inline]
/* FP:mod.rs-1402 */     pub fn inlined_instance<'tcx>(
/* FP:mod.rs-1403 */         self,
/* FP:mod.rs-1404 */         source_scopes: &IndexSlice<SourceScope, SourceScopeData<'tcx>>,
/* FP:mod.rs-1405 */     ) -> Option<ty::Instance<'tcx>> {
/* FP:mod.rs-1406 */         let scope_data = &source_scopes[self];
/* FP:mod.rs-1407 */         if let Some((inlined_instance, _)) = scope_data.inlined {
/* FP:mod.rs-1408 */             Some(inlined_instance)
/* FP:mod.rs-1409 */         } else if let Some(inlined_scope) = scope_data.inlined_parent_scope {
/* FP:mod.rs-1410 */             Some(source_scopes[inlined_scope].inlined.unwrap().0)
/* FP:mod.rs-1411 */         } else {
/* FP:mod.rs-1412 */             None
/* FP:mod.rs-1413 */         }
/* FP:mod.rs-1414 */     }
/* FP:mod.rs-1415 */ }
/* FP:mod.rs-1416 */ 
/* FP:mod.rs-1417 */ #[derive(Clone, Debug, TyEncodable, TyDecodable, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-1418 */ pub struct SourceScopeData<'tcx> {
/* FP:mod.rs-1419 */     pub span: Span,
/* FP:mod.rs-1420 */     pub parent_scope: Option<SourceScope>,
/* FP:mod.rs-1421 */ 
/* FP:mod.rs-1422 */     /// Whether this scope is the root of a scope tree of another body,
/* FP:mod.rs-1423 */     /// inlined into this body by the MIR inliner.
/* FP:mod.rs-1424 */     /// `ty::Instance` is the callee, and the `Span` is the call site.
/* FP:mod.rs-1425 */     pub inlined: Option<(ty::Instance<'tcx>, Span)>,
/* FP:mod.rs-1426 */ 
/* FP:mod.rs-1427 */     /// Nearest (transitive) parent scope (if any) which is inlined.
/* FP:mod.rs-1428 */     /// This is an optimization over walking up `parent_scope`
/* FP:mod.rs-1429 */     /// until a scope with `inlined: Some(...)` is found.
/* FP:mod.rs-1430 */     pub inlined_parent_scope: Option<SourceScope>,
/* FP:mod.rs-1431 */ 
/* FP:mod.rs-1432 */     /// Crate-local information for this source scope, that can't (and
/* FP:mod.rs-1433 */     /// needn't) be tracked across crates.
/* FP:mod.rs-1434 */     pub local_data: ClearCrossCrate<SourceScopeLocalData>,
/* FP:mod.rs-1435 */ }
/* FP:mod.rs-1436 */ 
/* FP:mod.rs-1437 */ #[derive(Clone, Debug, TyEncodable, TyDecodable, HashStable)]
/* FP:mod.rs-1438 */ pub struct SourceScopeLocalData {
/* FP:mod.rs-1439 */     /// An `HirId` with lint levels equivalent to this scope's lint levels.
/* FP:mod.rs-1440 */     pub lint_root: HirId,
/* FP:mod.rs-1441 */ }
/* FP:mod.rs-1442 */ 
/* FP:mod.rs-1443 */ /// A collection of projections into user types.
/* FP:mod.rs-1444 */ ///
/* FP:mod.rs-1445 */ /// They are projections because a binding can occur a part of a
/* FP:mod.rs-1446 */ /// parent pattern that has been ascribed a type.
/* FP:mod.rs-1447 */ ///
/* FP:mod.rs-1448 */ /// It's a collection because there can be multiple type ascriptions on
/* FP:mod.rs-1449 */ /// the path from the root of the pattern down to the binding itself.
/* FP:mod.rs-1450 */ ///
/* FP:mod.rs-1451 */ /// An example:
/* FP:mod.rs-1452 */ ///
/* FP:mod.rs-1453 */ /// ```ignore (illustrative)
/* FP:mod.rs-1454 */ /// struct S<'a>((i32, &'a str), String);
/* FP:mod.rs-1455 */ /// let S((_, w): (i32, &'static str), _): S = ...;
/* FP:mod.rs-1456 */ /// //    ------  ^^^^^^^^^^^^^^^^^^^ (1)
/* FP:mod.rs-1457 */ /// //  ---------------------------------  ^ (2)
/* FP:mod.rs-1458 */ /// ```
/* FP:mod.rs-1459 */ ///
/* FP:mod.rs-1460 */ /// The highlights labelled `(1)` show the subpattern `(_, w)` being
/* FP:mod.rs-1461 */ /// ascribed the type `(i32, &'static str)`.
/* FP:mod.rs-1462 */ ///
/* FP:mod.rs-1463 */ /// The highlights labelled `(2)` show the whole pattern being
/* FP:mod.rs-1464 */ /// ascribed the type `S`.
/* FP:mod.rs-1465 */ ///
/* FP:mod.rs-1466 */ /// In this example, when we descend to `w`, we will have built up the
/* FP:mod.rs-1467 */ /// following two projected types:
/* FP:mod.rs-1468 */ ///
/* FP:mod.rs-1469 */ ///   * base: `S`,                   projection: `(base.0).1`
/* FP:mod.rs-1470 */ ///   * base: `(i32, &'static str)`, projection: `base.1`
/* FP:mod.rs-1471 */ ///
/* FP:mod.rs-1472 */ /// The first will lead to the constraint `w: &'1 str` (for some
/* FP:mod.rs-1473 */ /// inferred region `'1`). The second will lead to the constraint `w:
/* FP:mod.rs-1474 */ /// &'static str`.
/* FP:mod.rs-1475 */ #[derive(Clone, Debug, TyEncodable, TyDecodable, HashStable, TypeFoldable, TypeVisitable)]
/* FP:mod.rs-1476 */ pub struct UserTypeProjections {
/* FP:mod.rs-1477 */     pub contents: Vec<UserTypeProjection>,
/* FP:mod.rs-1478 */ }
/* FP:mod.rs-1479 */ 
/* FP:mod.rs-1480 */ impl UserTypeProjections {
/* FP:mod.rs-1481 */     pub fn projections(&self) -> impl Iterator<Item = &UserTypeProjection> + ExactSizeIterator {
/* FP:mod.rs-1482 */         self.contents.iter()
/* FP:mod.rs-1483 */     }
/* FP:mod.rs-1484 */ }
/* FP:mod.rs-1485 */ 
/* FP:mod.rs-1486 */ /// Encodes the effect of a user-supplied type annotation on the
/* FP:mod.rs-1487 */ /// subcomponents of a pattern. The effect is determined by applying the
/* FP:mod.rs-1488 */ /// given list of projections to some underlying base type. Often,
/* FP:mod.rs-1489 */ /// the projection element list `projs` is empty, in which case this
/* FP:mod.rs-1490 */ /// directly encodes a type in `base`. But in the case of complex patterns with
/* FP:mod.rs-1491 */ /// subpatterns and bindings, we want to apply only a *part* of the type to a variable,
/* FP:mod.rs-1492 */ /// in which case the `projs` vector is used.
/* FP:mod.rs-1493 */ ///
/* FP:mod.rs-1494 */ /// Examples:
/* FP:mod.rs-1495 */ ///
/* FP:mod.rs-1496 */ /// * `let x: T = ...` -- here, the `projs` vector is empty.
/* FP:mod.rs-1497 */ ///
/* FP:mod.rs-1498 */ /// * `let (x, _): T = ...` -- here, the `projs` vector would contain
/* FP:mod.rs-1499 */ ///   `field[0]` (aka `.0`), indicating that the type of `s` is
/* FP:mod.rs-1500 */ ///   determined by finding the type of the `.0` field from `T`.
/* FP:mod.rs-1501 */ #[derive(Clone, Debug, TyEncodable, TyDecodable, Hash, HashStable, PartialEq)]
/* FP:mod.rs-1502 */ #[derive(TypeFoldable, TypeVisitable)]
/* FP:mod.rs-1503 */ pub struct UserTypeProjection {
/* FP:mod.rs-1504 */     pub base: UserTypeAnnotationIndex,
/* FP:mod.rs-1505 */     pub projs: Vec<ProjectionKind>,
/* FP:mod.rs-1506 */ }
/* FP:mod.rs-1507 */ 
/* FP:mod.rs-1508 */ crate::rustc_index::newtype_index! {
/* FP:mod.rs-1509 */     #[derive(HashStable)]
/* FP:mod.rs-1510 */     #[encodable]
/* FP:mod.rs-1511 */     #[orderable]
/* FP:mod.rs-1512 */     #[debug_format = "promoted[{}]"]
/* FP:mod.rs-1513 */     pub struct Promoted {}
/* FP:mod.rs-1514 */ }
/* FP:mod.rs-1515 */ 
/* FP:mod.rs-1516 */ /// `Location` represents the position of the start of the statement; or, if
/* FP:mod.rs-1517 */ /// `statement_index` equals the number of statements, then the start of the
/* FP:mod.rs-1518 */ /// terminator.
/* FP:mod.rs-1519 */ #[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, HashStable)]
/* FP:mod.rs-1520 */ pub struct Location {
/* FP:mod.rs-1521 */     /// The block that the location is within.
/* FP:mod.rs-1522 */     pub block: BasicBlock,
/* FP:mod.rs-1523 */ 
/* FP:mod.rs-1524 */     pub statement_index: usize,
/* FP:mod.rs-1525 */ }
/* FP:mod.rs-1526 */ 
/* FP:mod.rs-1527 */ impl fmt::Debug for Location {
/* FP:mod.rs-1528 */     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:mod.rs-1529 */         write!(fmt, "{:?}[{}]", self.block, self.statement_index)
/* FP:mod.rs-1530 */     }
/* FP:mod.rs-1531 */ }
/* FP:mod.rs-1532 */ 
/* FP:mod.rs-1533 */ impl Location {
/* FP:mod.rs-1534 */     pub const START: Location = Location { block: START_BLOCK, statement_index: 0 };
/* FP:mod.rs-1535 */ 
/* FP:mod.rs-1536 */     /// Returns the location immediately after this one within the enclosing block.
/* FP:mod.rs-1537 */     ///
/* FP:mod.rs-1538 */     /// Note that if this location represents a terminator, then the
/* FP:mod.rs-1539 */     /// resulting location would be out of bounds and invalid.
/* FP:mod.rs-1540 */     #[inline]
/* FP:mod.rs-1541 */     pub fn successor_within_block(&self) -> Location {
/* FP:mod.rs-1542 */         Location { block: self.block, statement_index: self.statement_index + 1 }
/* FP:mod.rs-1543 */     }
/* FP:mod.rs-1544 */ 
/* FP:mod.rs-1545 */     /// Returns `true` if `other` is earlier in the control flow graph than `self`.
/* FP:mod.rs-1546 */     pub fn is_predecessor_of<'tcx>(&self, other: Location, body: &Body<'tcx>) -> bool {
/* FP:mod.rs-1547 */         // If we are in the same block as the other location and are an earlier statement
/* FP:mod.rs-1548 */         // then we are a predecessor of `other`.
/* FP:mod.rs-1549 */         if self.block == other.block && self.statement_index < other.statement_index {
/* FP:mod.rs-1550 */             return true;
/* FP:mod.rs-1551 */         }
/* FP:mod.rs-1552 */ 
/* FP:mod.rs-1553 */         let predecessors = body.basic_blocks.predecessors();
/* FP:mod.rs-1554 */ 
/* FP:mod.rs-1555 */         // If we're in another block, then we want to check that block is a predecessor of `other`.
/* FP:mod.rs-1556 */         let mut queue: Vec<BasicBlock> = predecessors[other.block].to_vec();
/* FP:mod.rs-1557 */         let mut visited = FxHashSet::default();
/* FP:mod.rs-1558 */ 
/* FP:mod.rs-1559 */         while let Some(block) = queue.pop() {
/* FP:mod.rs-1560 */             // If we haven't visited this block before, then make sure we visit its predecessors.
/* FP:mod.rs-1561 */             if visited.insert(block) {
/* FP:mod.rs-1562 */                 queue.extend(predecessors[block].iter().cloned());
/* FP:mod.rs-1563 */             } else {
/* FP:mod.rs-1564 */                 continue;
/* FP:mod.rs-1565 */             }
/* FP:mod.rs-1566 */ 
/* FP:mod.rs-1567 */             // If we found the block that `self` is in, then we are a predecessor of `other` (since
/* FP:mod.rs-1568 */             // we found that block by looking at the predecessors of `other`).
/* FP:mod.rs-1569 */             if self.block == block {
/* FP:mod.rs-1570 */                 return true;
/* FP:mod.rs-1571 */             }
/* FP:mod.rs-1572 */         }
/* FP:mod.rs-1573 */ 
/* FP:mod.rs-1574 */         false
/* FP:mod.rs-1575 */     }
/* FP:mod.rs-1576 */ 
/* FP:mod.rs-1577 */     #[inline]
/* FP:mod.rs-1578 */     pub fn dominates(&self, other: Location, dominators: &Dominators<BasicBlock>) -> bool {
/* FP:mod.rs-1579 */         if self.block == other.block {
/* FP:mod.rs-1580 */             self.statement_index <= other.statement_index
/* FP:mod.rs-1581 */         } else {
/* FP:mod.rs-1582 */             dominators.dominates(self.block, other.block)
/* FP:mod.rs-1583 */         }
/* FP:mod.rs-1584 */     }
/* FP:mod.rs-1585 */ }
/* FP:mod.rs-1586 */ 
/* FP:mod.rs-1587 */ /// `DefLocation` represents the location of a definition - either an argument or an assignment
/* FP:mod.rs-1588 */ /// within MIR body.
/* FP:mod.rs-1589 */ #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/* FP:mod.rs-1590 */ pub enum DefLocation {
/* FP:mod.rs-1591 */     Argument,
/* FP:mod.rs-1592 */     Assignment(Location),
/* FP:mod.rs-1593 */     CallReturn { call: BasicBlock, target: Option<BasicBlock> },
/* FP:mod.rs-1594 */ }
/* FP:mod.rs-1595 */ 
/* FP:mod.rs-1596 */ impl DefLocation {
/* FP:mod.rs-1597 */     #[inline]
/* FP:mod.rs-1598 */     pub fn dominates(self, location: Location, dominators: &Dominators<BasicBlock>) -> bool {
/* FP:mod.rs-1599 */         match self {
/* FP:mod.rs-1600 */             DefLocation::Argument => true,
/* FP:mod.rs-1601 */             DefLocation::Assignment(def) => {
/* FP:mod.rs-1602 */                 def.successor_within_block().dominates(location, dominators)
/* FP:mod.rs-1603 */             }
/* FP:mod.rs-1604 */             DefLocation::CallReturn { target: None, .. } => false,
/* FP:mod.rs-1605 */             DefLocation::CallReturn { call, target: Some(target) } => {
/* FP:mod.rs-1606 */                 // The definition occurs on the call -> target edge. The definition dominates a use
/* FP:mod.rs-1607 */                 // if and only if the edge is on all paths from the entry to the use.
/* FP:mod.rs-1608 */                 //
/* FP:mod.rs-1609 */                 // Note that a call terminator has only one edge that can reach the target, so when
/* FP:mod.rs-1610 */                 // the call strongly dominates the target, all paths from the entry to the target
/* FP:mod.rs-1611 */                 // go through the call -> target edge.
/* FP:mod.rs-1612 */                 call != target
/* FP:mod.rs-1613 */                     && dominators.dominates(call, target)
/* FP:mod.rs-1614 */                     && dominators.dominates(target, location.block)
/* FP:mod.rs-1615 */             }
/* FP:mod.rs-1616 */         }
/* FP:mod.rs-1617 */     }
/* FP:mod.rs-1618 */ }
/* FP:mod.rs-1619 */ 
/* FP:mod.rs-1620 */ /// Checks if the specified `local` is used as the `self` parameter of a method call
/* FP:mod.rs-1621 */ /// in the provided `BasicBlock`. If it is, then the `DefId` of the called method is
/* FP:mod.rs-1622 */ /// returned.
/* FP:mod.rs-1623 */ pub fn find_self_call<'tcx>(
/* FP:mod.rs-1624 */     tcx: TyCtxt<'tcx>,
/* FP:mod.rs-1625 */     body: &Body<'tcx>,
/* FP:mod.rs-1626 */     local: Local,
/* FP:mod.rs-1627 */     block: BasicBlock,
/* FP:mod.rs-1628 */ ) -> Option<(DefId, GenericArgsRef<'tcx>)> {
/* FP:mod.rs-1629 */     debug!("find_self_call(local={:?}): terminator={:?}", local, body[block].terminator);
/* FP:mod.rs-1630 */     if let Some(Terminator { kind: TerminatorKind::Call { func, args, .. }, .. }) =
/* FP:mod.rs-1631 */         &body[block].terminator
/* FP:mod.rs-1632 */         && let Operand::Constant(box ConstOperand { const_, .. }) = func
/* FP:mod.rs-1633 */         && let ty::FnDef(def_id, fn_args) = *const_.ty().kind()
/* FP:mod.rs-1634 */         && let Some(item) = tcx.opt_associated_item(def_id)
/* FP:mod.rs-1635 */         && item.is_method()
/* FP:mod.rs-1636 */         && let [Spanned { node: Operand::Move(self_place) | Operand::Copy(self_place), .. }, ..] =
/* FP:mod.rs-1637 */             **args
/* FP:mod.rs-1638 */     {
/* FP:mod.rs-1639 */         if self_place.as_local() == Some(local) {
/* FP:mod.rs-1640 */             return Some((def_id, fn_args));
/* FP:mod.rs-1641 */         }
/* FP:mod.rs-1642 */ 
/* FP:mod.rs-1643 */         // Handle the case where `self_place` gets reborrowed.
/* FP:mod.rs-1644 */         // This happens when the receiver is `&T`.
/* FP:mod.rs-1645 */         for stmt in &body[block].statements {
/* FP:mod.rs-1646 */             if let StatementKind::Assign(box (place, rvalue)) = &stmt.kind
/* FP:mod.rs-1647 */                 && let Some(reborrow_local) = place.as_local()
/* FP:mod.rs-1648 */                 && self_place.as_local() == Some(reborrow_local)
/* FP:mod.rs-1649 */                 && let Rvalue::Ref(_, _, deref_place) = rvalue
/* FP:mod.rs-1650 */                 && let PlaceRef { local: deref_local, projection: [ProjectionElem::Deref] } =
/* FP:mod.rs-1651 */                     deref_place.as_ref()
/* FP:mod.rs-1652 */                 && deref_local == local
/* FP:mod.rs-1653 */             {
/* FP:mod.rs-1654 */                 return Some((def_id, fn_args));
/* FP:mod.rs-1655 */             }
/* FP:mod.rs-1656 */         }
/* FP:mod.rs-1657 */     }
/* FP:mod.rs-1658 */     None
/* FP:mod.rs-1659 */ }
/* FP:mod.rs-1660 */ 
/* FP:mod.rs-1661 */ // Some nodes are used a lot. Make sure they don't unintentionally get bigger.
/* FP:mod.rs-1662 */ #[cfg(target_pointer_width = "64")]
/* FP:mod.rs-1663 */ mod size_asserts {
/* FP:mod.rs-1664 */     use crate::rustc_data_structures::static_assert_size;
/* FP:mod.rs-1665 */ 
/* FP:mod.rs-1666 */     use super::*;
/* FP:mod.rs-1667 */     // tidy-alphabetical-start
/* FP:mod.rs-1668 */     static_assert_size!(BasicBlockData<'_>, 128);
/* FP:mod.rs-1669 */     static_assert_size!(LocalDecl<'_>, 40);
/* FP:mod.rs-1670 */     static_assert_size!(SourceScopeData<'_>, 64);
/* FP:mod.rs-1671 */     static_assert_size!(Statement<'_>, 32);
/* FP:mod.rs-1672 */     static_assert_size!(Terminator<'_>, 96);
/* FP:mod.rs-1673 */     static_assert_size!(VarDebugInfo<'_>, 88);
/* FP:mod.rs-1674 */     // tidy-alphabetical-end
/* FP:mod.rs-1675 */ }