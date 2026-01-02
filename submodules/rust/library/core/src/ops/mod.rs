mkmod!{arith, { 
                getname!(arith);
                getsrc!(arith);
                getpath!(arith);
                get_deps!(arith);
                get_crates!(arith);
                mkinclude!(arith);
                 
            }}
mkmod!{async_function, { 
                getname!(async_function);
                getsrc!(async_function);
                getpath!(async_function);
                get_deps!(async_function);
                get_crates!(async_function);
                mkinclude!(async_function);
                 
            }}
mkmod!{bit, { 
                getname!(bit);
                getsrc!(bit);
                getpath!(bit);
                get_deps!(bit);
                get_crates!(bit);
                mkinclude!(bit);
                 
            }}
mkmod!{control_flow, { 
                getname!(control_flow);
                getsrc!(control_flow);
                getpath!(control_flow);
                get_deps!(control_flow);
                get_crates!(control_flow);
                mkinclude!(control_flow);
                 
            }}
mkmod!{coroutine, { 
                getname!(coroutine);
                getsrc!(coroutine);
                getpath!(coroutine);
                get_deps!(coroutine);
                get_crates!(coroutine);
                mkinclude!(coroutine);
                 
            }}
mkmod!{deref, { 
                getname!(deref);
                getsrc!(deref);
                getpath!(deref);
                get_deps!(deref);
                get_crates!(deref);
                mkinclude!(deref);
                 
            }}
mkmod!{drop, { 
                getname!(drop);
                getsrc!(drop);
                getpath!(drop);
                get_deps!(drop);
                get_crates!(drop);
                mkinclude!(drop);
                 
            }}
mkmod!{function, { 
                getname!(function);
                getsrc!(function);
                getpath!(function);
                get_deps!(function);
                get_crates!(function);
                mkinclude!(function);
                 
            }}
mkmod!{index, { 
                getname!(index);
                getsrc!(index);
                getpath!(index);
                get_deps!(index);
                get_crates!(index);
                mkinclude!(index);
                 
            }}
mkmod!{index_range, { 
                getname!(index_range);
                getsrc!(index_range);
                getpath!(index_range);
                get_deps!(index_range);
                get_crates!(index_range);
                mkinclude!(index_range);
                 
            }}
mkmod!{range, { 
                getname!(range);
                getsrc!(range);
                getpath!(range);
                get_deps!(range);
                get_crates!(range);
                mkinclude!(range);
                 
            }}
mkmod!{try_trait, { 
                getname!(try_trait);
                getsrc!(try_trait);
                getpath!(try_trait);
                get_deps!(try_trait);
                get_crates!(try_trait);
                mkinclude!(try_trait);
                 
            }}
mkmod!{unsize, { 
                getname!(unsize);
                getsrc!(unsize);
                getpath!(unsize);
                get_deps!(unsize);
                get_crates!(unsize);
                mkinclude!(unsize);
                 
            }}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: arith :: { Add , Div , Mul , Neg , Rem , Sub } ;}
mkuse!{# [stable (feature = "op_assign_traits" , since = "1.8.0")] pub use self :: arith :: { AddAssign , DivAssign , MulAssign , RemAssign , SubAssign } ;}
mkuse!{# [unstable (feature = "async_fn_traits" , issue = "none")] pub use self :: async_function :: { AsyncFn , AsyncFnMut , AsyncFnOnce } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: bit :: { BitAnd , BitOr , BitXor , Not , Shl , Shr } ;}
mkuse!{# [stable (feature = "op_assign_traits" , since = "1.8.0")] pub use self :: bit :: { BitAndAssign , BitOrAssign , BitXorAssign , ShlAssign , ShrAssign } ;}
mkuse!{# [stable (feature = "control_flow_enum_type" , since = "1.55.0")] pub use self :: control_flow :: ControlFlow ;}
mkuse!{# [unstable (feature = "coroutine_trait" , issue = "43122")] pub use self :: coroutine :: { Coroutine , CoroutineState } ;}
mkuse!{# [unstable (feature = "deref_pure_trait" , issue = "87121")] pub use self :: deref :: DerefPure ;}
mkuse!{# [unstable (feature = "legacy_receiver_trait" , issue = "none")] pub use self :: deref :: LegacyReceiver ;}
mkuse!{# [unstable (feature = "arbitrary_self_types" , issue = "44874")] pub use self :: deref :: Receiver ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: deref :: { Deref , DerefMut } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: drop :: Drop ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: function :: { Fn , FnMut , FnOnce } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: index :: { Index , IndexMut } ;}
mkuse!{pub (crate) use self :: index_range :: IndexRange ;}
mkuse!{# [unstable (feature = "range_into_bounds" , issue = "136903")] pub use self :: range :: IntoBounds ;}
mkuse!{# [stable (feature = "inclusive_range" , since = "1.26.0")] pub use self :: range :: { Bound , RangeBounds , RangeInclusive , RangeToInclusive } ;}
mkuse!{# [unstable (feature = "one_sided_range" , issue = "69780")] pub use self :: range :: { OneSidedRange , OneSidedRangeBound } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: range :: { Range , RangeFrom , RangeFull , RangeTo } ;}
mkuse!{# [unstable (feature = "try_trait_v2_residual" , issue = "91285")] pub use self :: try_trait :: Residual ;}
mkuse!{# [unstable (feature = "try_trait_v2_yeet" , issue = "96374")] pub use self :: try_trait :: Yeet ;}
mkuse!{pub (crate) use self :: try_trait :: { ChangeOutputType , NeverShortCircuit } ;}
mkuse!{# [unstable (feature = "try_trait_v2" , issue = "84277" , old_name = "try_trait")] pub use self :: try_trait :: { FromResidual , Try } ;}
mkuse!{# [unstable (feature = "coerce_unsized" , issue = "18598")] pub use self :: unsize :: CoerceUnsized ;}
mkuse!{# [unstable (feature = "dispatch_from_dyn" , issue = "none")] pub use self :: unsize :: DispatchFromDyn ;}