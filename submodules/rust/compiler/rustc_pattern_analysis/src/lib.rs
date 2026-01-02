mkmod!{checks, { 
                getname!(checks);
                getsrc!(checks);
                getpath!(checks);
                get_deps!(checks);
                get_crates!(checks);
                mkinclude!(checks);
                 
            }}
mkmod!{constructor, { 
                getname!(constructor);
                getsrc!(constructor);
                getpath!(constructor);
                get_deps!(constructor);
                get_crates!(constructor);
                mkinclude!(constructor);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{lints, { 
                getname!(lints);
                getsrc!(lints);
                getpath!(lints);
                get_deps!(lints);
                get_crates!(lints);
                mkinclude!(lints);
                 
            }}
mkmod!{pat, { 
                getname!(pat);
                getsrc!(pat);
                getpath!(pat);
                get_deps!(pat);
                get_crates!(pat);
                mkinclude!(pat);
                 
            }}
mkmod!{pat_column, { 
                getname!(pat_column);
                getsrc!(pat_column);
                getpath!(pat_column);
                get_deps!(pat_column);
                get_crates!(pat_column);
                mkinclude!(pat_column);
                 
            }}
mkmod!{rustc, { 
                getname!(rustc);
                getsrc!(rustc);
                getpath!(rustc);
                get_deps!(rustc);
                get_crates!(rustc);
                mkinclude!(rustc);
                 
            }}
mkmod!{usefulness, { 
                getname!(usefulness);
                getsrc!(usefulness);
                getpath!(usefulness);
                get_deps!(usefulness);
                get_crates!(usefulness);
                mkinclude!(usefulness);
                 
            }}
mkitem!{# [cfg (feature = "rustc")] rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}
mkuse!{use std :: fmt ;}
mkuse!{pub use rustc_index :: { Idx , IndexVec } ;}
mkuse!{use crate :: constructor :: { Constructor , ConstructorSet , IntRange } ;}
mkuse!{use crate :: pat :: DeconstructedPat ;}
mkitem!{mktrait!{pub trait Captures < 'a > { }}}
mkitem!{mkimpl!{impl < 'a , T : ? Sized > Captures < 'a > for T { }}}
mkitem!{mkstruct!{# [doc = " `bool` newtype that indicates whether this is a privately uninhabited field that we should skip"] # [doc = " during analysis."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct PrivateUninhabitedField (pub bool) ;}}
mkitem!{mktrait!{# [doc = " Context that provides type information about constructors."] # [doc = ""] # [doc = " Most of the crate is parameterized on a type that implements this trait."] pub trait PatCx : Sized + fmt :: Debug { # [doc = " The type of a pattern."] type Ty : Clone + fmt :: Debug ; # [doc = " Errors that can abort analysis."] type Error : fmt :: Debug ; # [doc = " The index of an enum variant."] type VariantIdx : Clone + Idx + fmt :: Debug ; # [doc = " A string literal"] type StrLit : Clone + PartialEq + fmt :: Debug ; # [doc = " Extra data to store in a match arm."] type ArmData : Copy + Clone + fmt :: Debug ; # [doc = " Extra data to store in a pattern."] type PatData : Clone ; fn is_exhaustive_patterns_feature_on (& self) -> bool ; # [doc = " Whether to ensure the non-exhaustiveness witnesses we report for a complete set. This is"] # [doc = " `false` by default to avoid some exponential blowup cases such as"] # [doc = " <https://github.com/rust-lang/rust/issues/118437>."] fn exhaustive_witnesses (& self) -> bool { false } # [doc = " The number of fields for this constructor."] fn ctor_arity (& self , ctor : & Constructor < Self > , ty : & Self :: Ty) -> usize ; # [doc = " The types of the fields for this constructor. The result must contain `ctor_arity()` fields."] fn ctor_sub_tys (& self , ctor : & Constructor < Self > , ty : & Self :: Ty ,) -> impl Iterator < Item = (Self :: Ty , PrivateUninhabitedField) > + ExactSizeIterator ; # [doc = " The set of all the constructors for `ty`."] # [doc = ""] # [doc = " This must follow the invariants of `ConstructorSet`"] fn ctors_for_ty (& self , ty : & Self :: Ty) -> Result < ConstructorSet < Self > , Self :: Error > ; # [doc = " Write the name of the variant represented by `pat`. Used for the best-effort `Debug` impl of"] # [doc = " `DeconstructedPat`. Only invoqued when `pat.ctor()` is `Struct | Variant(_) | UnionField`."] fn write_variant_name (f : & mut fmt :: Formatter < '_ > , ctor : & crate :: constructor :: Constructor < Self > , ty : & Self :: Ty ,) -> fmt :: Result ; # [doc = " Raise a bug."] fn bug (& self , fmt : fmt :: Arguments < '_ >) -> Self :: Error ; # [doc = " Lint that the range `pat` overlapped with all the ranges in `overlaps_with`, where the range"] # [doc = " they overlapped over is `overlaps_on`. We only detect singleton overlaps."] # [doc = " The default implementation does nothing."] fn lint_overlapping_range_endpoints (& self , _pat : & DeconstructedPat < Self > , _overlaps_on : IntRange , _overlaps_with : & [& DeconstructedPat < Self >] ,) { } # [doc = " The maximum pattern complexity limit was reached."] fn complexity_exceeded (& self) -> Result < () , Self :: Error > ; # [doc = " Lint that there is a gap `gap` between `pat` and all of `gapped_with` such that the gap is"] # [doc = " not matched by another range. If `gapped_with` is empty, then `gap` is `T::MAX`. We only"] # [doc = " detect singleton gaps."] # [doc = " The default implementation does nothing."] fn lint_non_contiguous_range_endpoints (& self , _pat : & DeconstructedPat < Self > , _gap : IntRange , _gapped_with : & [& DeconstructedPat < Self >] ,) { } # [doc = " Check if we may need to perform additional deref-pattern-specific validation."] fn match_may_contain_deref_pats (& self) -> bool { true } # [doc = " The current implementation of deref patterns requires that they can't match on the same"] # [doc = " place as a normal constructor. Since this isn't caught by type-checking, we check it in the"] # [doc = " `PatCx` before running the analysis. This reports an error if the check fails."] fn report_mixed_deref_pat_ctors (& self , deref_pat : & DeconstructedPat < Self > , normal_pat : & DeconstructedPat < Self > ,) -> Self :: Error ; }}}
mkitem!{mkstruct!{# [doc = " The arm of a match expression."] # [derive (Debug)] pub struct MatchArm < 'p , Cx : PatCx > { pub pat : & 'p DeconstructedPat < Cx > , pub has_guard : bool , pub arm_data : Cx :: ArmData , }}}
mkitem!{mkimpl!{impl < 'p , Cx : PatCx > Clone for MatchArm < 'p , Cx > { fn clone (& self) -> Self { Self { pat : self . pat , has_guard : self . has_guard , arm_data : self . arm_data } } }}}
mkitem!{mkimpl!{impl < 'p , Cx : PatCx > Copy for MatchArm < 'p , Cx > { }}}