mkitem!{extern crate self as rustc_type_ir ;}
mkuse!{use std :: fmt ;}
mkuse!{use std :: hash :: Hash ;}
mkuse!{# [cfg (feature = "nightly")] use rustc_macros :: { Decodable , Encodable , HashStable_NoContext } ;}
mkmod!{data_structures, { 
                getname!(data_structures);
                getsrc!(data_structures);
                getpath!(data_structures);
                get_deps!(data_structures);
                get_crates!(data_structures);
                mkinclude!(data_structures);
                 
            }}
mkmod!{elaborate, { 
                getname!(elaborate);
                getsrc!(elaborate);
                getpath!(elaborate);
                get_deps!(elaborate);
                get_crates!(elaborate);
                mkinclude!(elaborate);
                 
            }}
mkmod!{error, { 
                getname!(error);
                getsrc!(error);
                getpath!(error);
                get_deps!(error);
                get_crates!(error);
                mkinclude!(error);
                 
            }}
mkmod!{fast_reject, { 
                getname!(fast_reject);
                getsrc!(fast_reject);
                getpath!(fast_reject);
                get_deps!(fast_reject);
                get_crates!(fast_reject);
                mkinclude!(fast_reject);
                 
            }}
mkmod!{inherent, { 
                getname!(inherent);
                getsrc!(inherent);
                getpath!(inherent);
                get_deps!(inherent);
                get_crates!(inherent);
                mkinclude!(inherent);
                 
            }}
mkmod!{ir_print, { 
                getname!(ir_print);
                getsrc!(ir_print);
                getpath!(ir_print);
                get_deps!(ir_print);
                get_crates!(ir_print);
                mkinclude!(ir_print);
                 
            }}
mkmod!{lang_items, { 
                getname!(lang_items);
                getsrc!(lang_items);
                getpath!(lang_items);
                get_deps!(lang_items);
                get_crates!(lang_items);
                mkinclude!(lang_items);
                 
            }}
mkmod!{lift, { 
                getname!(lift);
                getsrc!(lift);
                getpath!(lift);
                get_deps!(lift);
                get_crates!(lift);
                mkinclude!(lift);
                 
            }}
mkmod!{outlives, { 
                getname!(outlives);
                getsrc!(outlives);
                getpath!(outlives);
                get_deps!(outlives);
                get_crates!(outlives);
                mkinclude!(outlives);
                 
            }}
mkmod!{relate, { 
                getname!(relate);
                getsrc!(relate);
                getpath!(relate);
                get_deps!(relate);
                get_crates!(relate);
                mkinclude!(relate);
                 
            }}
mkmod!{search_graph, { 
                getname!(search_graph);
                getsrc!(search_graph);
                getpath!(search_graph);
                get_deps!(search_graph);
                get_crates!(search_graph);
                mkinclude!(search_graph);
                 
            }}
mkmod!{solve, { 
                getname!(solve);
                getsrc!(solve);
                getpath!(solve);
                get_deps!(solve);
                get_crates!(solve);
                mkinclude!(solve);
                 
            }}
mkmod!{walk, { 
                getname!(walk);
                getsrc!(walk);
                getpath!(walk);
                get_deps!(walk);
                get_crates!(walk);
                mkinclude!(walk);
                 
            }}
mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{binder, { 
                getname!(binder);
                getsrc!(binder);
                getpath!(binder);
                get_deps!(binder);
                get_crates!(binder);
                mkinclude!(binder);
                 
            }}
mkmod!{canonical, { 
                getname!(canonical);
                getsrc!(canonical);
                getpath!(canonical);
                get_deps!(canonical);
                get_crates!(canonical);
                mkinclude!(canonical);
                 
            }}
mkmod!{const_kind, { 
                getname!(const_kind);
                getsrc!(const_kind);
                getpath!(const_kind);
                get_deps!(const_kind);
                get_crates!(const_kind);
                mkinclude!(const_kind);
                 
            }}
mkmod!{flags, { 
                getname!(flags);
                getsrc!(flags);
                getpath!(flags);
                get_deps!(flags);
                get_crates!(flags);
                mkinclude!(flags);
                 
            }}
mkmod!{fold, { 
                getname!(fold);
                getsrc!(fold);
                getpath!(fold);
                get_deps!(fold);
                get_crates!(fold);
                mkinclude!(fold);
                 
            }}
mkmod!{generic_arg, { 
                getname!(generic_arg);
                getsrc!(generic_arg);
                getpath!(generic_arg);
                get_deps!(generic_arg);
                get_crates!(generic_arg);
                mkinclude!(generic_arg);
                 
            }}
mkmod!{infer_ctxt, { 
                getname!(infer_ctxt);
                getsrc!(infer_ctxt);
                getpath!(infer_ctxt);
                get_deps!(infer_ctxt);
                get_crates!(infer_ctxt);
                mkinclude!(infer_ctxt);
                 
            }}
mkmod!{interner, { 
                getname!(interner);
                getsrc!(interner);
                getpath!(interner);
                get_deps!(interner);
                get_crates!(interner);
                mkinclude!(interner);
                 
            }}
mkmod!{opaque_ty, { 
                getname!(opaque_ty);
                getsrc!(opaque_ty);
                getpath!(opaque_ty);
                get_deps!(opaque_ty);
                get_crates!(opaque_ty);
                mkinclude!(opaque_ty);
                 
            }}
mkmod!{pattern, { 
                getname!(pattern);
                getsrc!(pattern);
                getpath!(pattern);
                get_deps!(pattern);
                get_crates!(pattern);
                mkinclude!(pattern);
                 
            }}
mkmod!{predicate, { 
                getname!(predicate);
                getsrc!(predicate);
                getpath!(predicate);
                get_deps!(predicate);
                get_crates!(predicate);
                mkinclude!(predicate);
                 
            }}
mkmod!{predicate_kind, { 
                getname!(predicate_kind);
                getsrc!(predicate_kind);
                getpath!(predicate_kind);
                get_deps!(predicate_kind);
                get_crates!(predicate_kind);
                mkinclude!(predicate_kind);
                 
            }}
mkmod!{region_kind, { 
                getname!(region_kind);
                getsrc!(region_kind);
                getpath!(region_kind);
                get_deps!(region_kind);
                get_crates!(region_kind);
                mkinclude!(region_kind);
                 
            }}
mkmod!{ty_info, { 
                getname!(ty_info);
                getsrc!(ty_info);
                getpath!(ty_info);
                get_deps!(ty_info);
                get_crates!(ty_info);
                mkinclude!(ty_info);
                 
            }}
mkmod!{ty_kind, { 
                getname!(ty_kind);
                getsrc!(ty_kind);
                getpath!(ty_kind);
                get_deps!(ty_kind);
                get_crates!(ty_kind);
                mkinclude!(ty_kind);
                 
            }}
mkmod!{upcast, { 
                getname!(upcast);
                getsrc!(upcast);
                getpath!(upcast);
                get_deps!(upcast);
                get_crates!(upcast);
                mkinclude!(upcast);
                 
            }}
mkmod!{visit, { 
                getname!(visit);
                getsrc!(visit);
                getpath!(visit);
                get_deps!(visit);
                get_crates!(visit);
                mkinclude!(visit);
                 
            }}
mkuse!{pub use AliasTyKind :: * ;}
mkuse!{pub use DynKind :: * ;}
mkuse!{pub use InferTy :: * ;}
mkuse!{pub use RegionKind :: * ;}
mkuse!{pub use TyKind :: * ;}
mkuse!{pub use Variance :: * ;}
mkuse!{pub use binder :: * ;}
mkuse!{pub use canonical :: * ;}
mkuse!{pub use const_kind :: * ;}
mkuse!{pub use flags :: * ;}
mkuse!{pub use fold :: * ;}
mkuse!{pub use generic_arg :: * ;}
mkuse!{pub use infer_ctxt :: * ;}
mkuse!{pub use interner :: * ;}
mkuse!{pub use opaque_ty :: * ;}
mkuse!{pub use pattern :: * ;}
mkuse!{pub use predicate :: * ;}
mkuse!{pub use predicate_kind :: * ;}
mkuse!{pub use region_kind :: * ;}
mkuse!{pub use rustc_ast_ir :: { FloatTy , IntTy , Movability , Mutability , Pinnedness , UintTy } ;}
mkuse!{pub use ty_info :: * ;}
mkuse!{pub use ty_kind :: * ;}
mkuse!{pub use upcast :: * ;}
mkuse!{pub use visit :: * ;}
mkitem!{rustc_index :: newtype_index ! { # [doc = " A [De Bruijn index][dbi] is a standard means of representing"] # [doc = " regions (and perhaps later types) in a higher-ranked setting. In"] # [doc = " particular, imagine a type like this:"] # [doc = " ```ignore (illustrative)"] # [doc = "    for<'a> fn(for<'b> fn(&'b isize, &'a isize), &'a char)"] # [doc = " // ^          ^            |          |           |"] # [doc = " // |          |            |          |           |"] # [doc = " // |          +------------+ 0        |           |"] # [doc = " // |                                  |           |"] # [doc = " // +----------------------------------+ 1         |"] # [doc = " // |                                              |"] # [doc = " // +----------------------------------------------+ 0"] # [doc = " ```"] # [doc = " In this type, there are two binders (the outer fn and the inner"] # [doc = " fn). We need to be able to determine, for any given region, which"] # [doc = " fn type it is bound by, the inner or the outer one. There are"] # [doc = " various ways you can do this, but a De Bruijn index is one of the"] # [doc = " more convenient and has some nice properties. The basic idea is to"] # [doc = " count the number of binders, inside out. Some examples should help"] # [doc = " clarify what I mean."] # [doc = ""] # [doc = " Let's start with the reference type `&'b isize` that is the first"] # [doc = " argument to the inner function. This region `'b` is assigned a De"] # [doc = " Bruijn index of 0, meaning \"the innermost binder\" (in this case, a"] # [doc = " fn). The region `'a` that appears in the second argument type (`&'a"] # [doc = " isize`) would then be assigned a De Bruijn index of 1, meaning \"the"] # [doc = " second-innermost binder\". (These indices are written on the arrows"] # [doc = " in the diagram)."] # [doc = ""] # [doc = " What is interesting is that De Bruijn index attached to a particular"] # [doc = " variable will vary depending on where it appears. For example,"] # [doc = " the final type `&'a char` also refers to the region `'a` declared on"] # [doc = " the outermost fn. But this time, this reference is not nested within"] # [doc = " any other binders (i.e., it is not an argument to the inner fn, but"] # [doc = " rather the outer one). Therefore, in this case, it is assigned a"] # [doc = " De Bruijn index of 0, because the innermost binder in that location"] # [doc = " is the outer fn."] # [doc = ""] # [doc = " [dbi]: https://en.wikipedia.org/wiki/De_Bruijn_index"] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] # [encodable] # [orderable] # [debug_format = "DebruijnIndex({})"] # [gate_rustc_only] pub struct DebruijnIndex { const INNERMOST = 0 ; } }}
mkitem!{mkimpl!{impl DebruijnIndex { # [doc = " Returns the resulting index when this value is moved into"] # [doc = " `amount` number of new binders. So, e.g., if you had"] # [doc = ""] # [doc = "    for<'a> fn(&'a x)"] # [doc = ""] # [doc = " and you wanted to change it to"] # [doc = ""] # [doc = "    for<'a> fn(for<'b> fn(&'a x))"] # [doc = ""] # [doc = " you would need to shift the index for `'a` into a new binder."] # [inline] # [must_use] pub fn shifted_in (self , amount : u32) -> DebruijnIndex { DebruijnIndex :: from_u32 (self . as_u32 () + amount) } # [doc = " Update this index in place by shifting it \"in\" through"] # [doc = " `amount` number of binders."] # [inline] pub fn shift_in (& mut self , amount : u32) { * self = self . shifted_in (amount) ; } # [doc = " Returns the resulting index when this value is moved out from"] # [doc = " `amount` number of new binders."] # [inline] # [must_use] pub fn shifted_out (self , amount : u32) -> DebruijnIndex { DebruijnIndex :: from_u32 (self . as_u32 () - amount) } # [doc = " Update in place by shifting out from `amount` binders."] # [inline] pub fn shift_out (& mut self , amount : u32) { * self = self . shifted_out (amount) ; } # [doc = " Adjusts any De Bruijn indices so as to make `to_binder` the"] # [doc = " innermost binder. That is, if we have something bound at `to_binder`,"] # [doc = " it will now be bound at INNERMOST. This is an appropriate thing to do"] # [doc = " when moving a region out from inside binders:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = "             for<'a>   fn(for<'b>   for<'c>   fn(&'a u32), _)"] # [doc = " // Binder:  D3           D2        D1            ^^"] # [doc = " ```"] # [doc = ""] # [doc = " Here, the region `'a` would have the De Bruijn index D3,"] # [doc = " because it is the bound 3 binders out. However, if we wanted"] # [doc = " to refer to that region `'a` in the second argument (the `_`),"] # [doc = " those two binders would not be in scope. In that case, we"] # [doc = " might invoke `shift_out_to_binder(D3)`. This would adjust the"] # [doc = " De Bruijn index of `'a` to D1 (the innermost binder)."] # [doc = ""] # [doc = " If we invoke `shift_out_to_binder` and the region is in fact"] # [doc = " bound by one of the binders we are shifting out of, that is an"] # [doc = " error (and should fail an assertion failure)."] # [inline] pub fn shifted_out_to_binder (self , to_binder : DebruijnIndex) -> Self { self . shifted_out (to_binder . as_u32 () - INNERMOST . as_u32 ()) } }}}

macro_rules! debug_bound_var_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function debug_bound_var in module {}", module_path!());
    };
}

mkfn!{
    debug_bound_var_introspect!();
    pub fn debug_bound_var < T : std :: fmt :: Write > (fmt : & mut T , debruijn : DebruijnIndex , var : impl std :: fmt :: Debug ,) -> Result < () , std :: fmt :: Error > { if debruijn == INNERMOST { write ! (fmt , "^{var:?}") } else { write ! (fmt , "^{}_{:?}" , debruijn . index () , var) } }
}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "nightly" , derive (Decodable , Encodable , HashStable_NoContext))] # [cfg_attr (feature = "nightly" , rustc_pass_by_value)] pub enum Variance { Covariant , Invariant , Contravariant , Bivariant , }}}
mkitem!{mkimpl!{impl Variance { # [doc = " `a.xform(b)` combines the variance of a context with the"] # [doc = " variance of a type with the following meaning. If we are in a"] # [doc = " context with variance `a`, and we encounter a type argument in"] # [doc = " a position with variance `b`, then `a.xform(b)` is the new"] # [doc = " variance with which the argument appears."] # [doc = ""] # [doc = " Example 1:"] # [doc = " ```ignore (illustrative)"] # [doc = " *mut Vec<i32>"] # [doc = " ```"] # [doc = " Here, the \"ambient\" variance starts as covariant. `*mut T` is"] # [doc = " invariant with respect to `T`, so the variance in which the"] # [doc = " `Vec<i32>` appears is `Covariant.xform(Invariant)`, which"] # [doc = " yields `Invariant`. Now, the type `Vec<T>` is covariant with"] # [doc = " respect to its type argument `T`, and hence the variance of"] # [doc = " the `i32` here is `Invariant.xform(Covariant)`, which results"] # [doc = " (again) in `Invariant`."] # [doc = ""] # [doc = " Example 2:"] # [doc = " ```ignore (illustrative)"] # [doc = " fn(*const Vec<i32>, *mut Vec<i32)"] # [doc = " ```"] # [doc = " The ambient variance is covariant. A `fn` type is"] # [doc = " contravariant with respect to its parameters, so the variance"] # [doc = " within which both pointer types appear is"] # [doc = " `Covariant.xform(Contravariant)`, or `Contravariant`. `*const"] # [doc = " T` is covariant with respect to `T`, so the variance within"] # [doc = " which the first `Vec<i32>` appears is"] # [doc = " `Contravariant.xform(Covariant)` or `Contravariant`. The same"] # [doc = " is true for its `i32` argument. In the `*mut T` case, the"] # [doc = " variance of `Vec<i32>` is `Contravariant.xform(Invariant)`,"] # [doc = " and hence the outermost type is `Invariant` with respect to"] # [doc = " `Vec<i32>` (and its `i32` argument)."] # [doc = ""] # [doc = " Source: Figure 1 of \"Taming the Wildcards:"] # [doc = " Combining Definition- and Use-Site Variance\" published in PLDI'11."] pub fn xform (self , v : Variance) -> Variance { match (self , v) { (Variance :: Covariant , Variance :: Covariant) => Variance :: Covariant , (Variance :: Covariant , Variance :: Contravariant) => Variance :: Contravariant , (Variance :: Covariant , Variance :: Invariant) => Variance :: Invariant , (Variance :: Covariant , Variance :: Bivariant) => Variance :: Bivariant , (Variance :: Contravariant , Variance :: Covariant) => Variance :: Contravariant , (Variance :: Contravariant , Variance :: Contravariant) => Variance :: Covariant , (Variance :: Contravariant , Variance :: Invariant) => Variance :: Invariant , (Variance :: Contravariant , Variance :: Bivariant) => Variance :: Bivariant , (Variance :: Invariant , _) => Variance :: Invariant , (Variance :: Bivariant , _) => Variance :: Bivariant , } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Variance { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match * self { Variance :: Covariant => "+" , Variance :: Contravariant => "-" , Variance :: Invariant => "o" , Variance :: Bivariant => "*" , }) } }}}
mkitem!{rustc_index :: newtype_index ! { # [doc = " \"Universes\" are used during type- and trait-checking in the"] # [doc = " presence of `for<..>` binders to control what sets of names are"] # [doc = " visible. Universes are arranged into a tree: the root universe"] # [doc = " contains names that are always visible. Each child then adds a new"] # [doc = " set of names that are visible, in addition to those of its parent."] # [doc = " We say that the child universe \"extends\" the parent universe with"] # [doc = " new names."] # [doc = ""] # [doc = " To make this more concrete, consider this program:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " struct Foo { }"] # [doc = " fn bar<T>(x: T) {"] # [doc = "   let y: for<'a> fn(&'a u8, Foo) = ...;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The struct name `Foo` is in the root universe U0. But the type"] # [doc = " parameter `T`, introduced on `bar`, is in an extended universe U1"] # [doc = " -- i.e., within `bar`, we can name both `T` and `Foo`, but outside"] # [doc = " of `bar`, we cannot name `T`. Then, within the type of `y`, the"] # [doc = " region `'a` is in a universe U2 that extends U1, because we can"] # [doc = " name it inside the fn type but not outside."] # [doc = ""] # [doc = " Universes are used to do type- and trait-checking around these"] # [doc = " \"forall\" binders (also called **universal quantification**). The"] # [doc = " idea is that when, in the body of `bar`, we refer to `T` as a"] # [doc = " type, we aren't referring to any type in particular, but rather a"] # [doc = " kind of \"fresh\" type that is distinct from all other types we have"] # [doc = " actually declared. This is called a **placeholder** type, and we"] # [doc = " use universes to talk about this. In other words, a type name in"] # [doc = " universe 0 always corresponds to some \"ground\" type that the user"] # [doc = " declared, but a type name in a non-zero universe is a placeholder"] # [doc = " type -- an idealized representative of \"types in general\" that we"] # [doc = " use for checking generic functions."] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] # [encodable] # [orderable] # [debug_format = "U{}"] # [gate_rustc_only] pub struct UniverseIndex { } }}
mkitem!{mkimpl!{impl UniverseIndex { pub const ROOT : UniverseIndex = UniverseIndex :: ZERO ; # [doc = " Returns the \"next\" universe index in order -- this new index"] # [doc = " is considered to extend all previous universes. This"] # [doc = " corresponds to entering a `forall` quantifier. So, for"] # [doc = " example, suppose we have this type in universe `U`:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " for<'a> fn(&'a u32)"] # [doc = " ```"] # [doc = ""] # [doc = " Once we \"enter\" into this `for<'a>` quantifier, we are in a"] # [doc = " new universe that extends `U` -- in this new universe, we can"] # [doc = " name the region `'a`, but that region was not nameable from"] # [doc = " `U` because it was not in scope there."] pub fn next_universe (self) -> UniverseIndex { UniverseIndex :: from_u32 (self . as_u32 () . checked_add (1) . unwrap ()) } # [doc = " Returns `true` if `self` can name a name from `other` -- in other words,"] # [doc = " if the set of names in `self` is a superset of those in"] # [doc = " `other` (`self >= other`)."] pub fn can_name (self , other : UniverseIndex) -> bool { self >= other } # [doc = " Returns `true` if `self` cannot name some names from `other` -- in other"] # [doc = " words, if the set of names in `self` is a strict subset of"] # [doc = " those in `other` (`self < other`)."] pub fn cannot_name (self , other : UniverseIndex) -> bool { self < other } # [doc = " Returns `true` if `self` is the root universe, otherwise false."] pub fn is_root (self) -> bool { self == Self :: ROOT } }}}
mkitem!{mkimpl!{impl Default for UniverseIndex { fn default () -> Self { Self :: ROOT } }}}
mkitem!{rustc_index :: newtype_index ! { # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] # [encodable] # [orderable] # [debug_format = "{}"] # [gate_rustc_only] pub struct BoundVar { } }}
mkitem!{mkenum!{# [doc = " Represents the various closure traits in the language. This"] # [doc = " will determine the type of the environment (`self`, in the"] # [doc = " desugaring) argument that the closure expects."] # [doc = ""] # [doc = " You can get the environment type of a closure using"] # [doc = " `tcx.closure_env_ty()`."] # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (Encodable , Decodable , HashStable_NoContext))] pub enum ClosureKind { Fn , FnMut , FnOnce , }}}
mkitem!{mkimpl!{impl ClosureKind { # [doc = " This is the initial value used when doing upvar inference."] pub const LATTICE_BOTTOM : ClosureKind = ClosureKind :: Fn ; pub const fn as_str (self) -> & 'static str { match self { ClosureKind :: Fn => "Fn" , ClosureKind :: FnMut => "FnMut" , ClosureKind :: FnOnce => "FnOnce" , } } # [doc = " Returns `true` if a type that impls this closure kind"] # [doc = " must also implement `other`."] # [rustfmt :: skip] pub fn extends (self , other : ClosureKind) -> bool { use ClosureKind :: * ; match (self , other) { (Fn , Fn | FnMut | FnOnce) | (FnMut , FnMut | FnOnce) | (FnOnce , FnOnce) => true , _ => false , } } }}}
mkitem!{mkimpl!{impl fmt :: Display for ClosureKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_str () . fmt (f) } }}}