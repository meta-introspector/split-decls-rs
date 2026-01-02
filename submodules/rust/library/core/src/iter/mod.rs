mkitem!{macro_rules ! impl_fold_via_try_fold { (fold -> try_fold) => { impl_fold_via_try_fold ! { @ internal fold -> try_fold } } ; (rfold -> try_rfold) => { impl_fold_via_try_fold ! { @ internal rfold -> try_rfold } } ; (spec_fold -> spec_try_fold) => { impl_fold_via_try_fold ! { @ internal spec_fold -> spec_try_fold } } ; (spec_rfold -> spec_try_rfold) => { impl_fold_via_try_fold ! { @ internal spec_rfold -> spec_try_rfold } } ; (@ internal $ fold : ident -> $ try_fold : ident) => { # [inline] fn $ fold < AAA , FFF > (mut self , init : AAA , fold : FFF) -> AAA where FFF : FnMut (AAA , Self :: Item) -> AAA , { use crate :: ops :: NeverShortCircuit ; self .$ try_fold (init , NeverShortCircuit :: wrap_mut_2 (fold)) . 0 } } ; }}
mkuse!{# [unstable (feature = "iter_array_chunks" , reason = "recently added" , issue = "100450")] pub use self :: adapters :: ArrayChunks ;}
mkuse!{# [unstable (feature = "std_internals" , issue = "none")] pub use self :: adapters :: ByRefSized ;}
mkuse!{# [stable (feature = "iter_cloned" , since = "1.1.0")] pub use self :: adapters :: Cloned ;}
mkuse!{# [stable (feature = "iter_copied" , since = "1.36.0")] pub use self :: adapters :: Copied ;}
mkuse!{# [stable (feature = "iterator_flatten" , since = "1.29.0")] pub use self :: adapters :: Flatten ;}
mkuse!{# [stable (feature = "iter_map_while" , since = "1.57.0")] pub use self :: adapters :: MapWhile ;}
mkuse!{# [unstable (feature = "iter_map_windows" , reason = "recently added" , issue = "87155")] pub use self :: adapters :: MapWindows ;}
mkuse!{# [unstable (feature = "inplace_iteration" , issue = "none")] pub use self :: adapters :: SourceIter ;}
mkuse!{# [stable (feature = "iterator_step_by" , since = "1.28.0")] pub use self :: adapters :: StepBy ;}
mkuse!{# [unstable (feature = "trusted_random_access" , issue = "none")] pub use self :: adapters :: TrustedRandomAccess ;}
mkuse!{# [unstable (feature = "trusted_random_access" , issue = "none")] pub use self :: adapters :: TrustedRandomAccessNoCoerce ;}
mkuse!{# [stable (feature = "iter_chain" , since = "1.91.0")] pub use self :: adapters :: chain ;}
mkuse!{pub (crate) use self :: adapters :: try_process ;}
mkuse!{# [stable (feature = "iter_zip" , since = "1.59.0")] pub use self :: adapters :: zip ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: adapters :: { Chain , Cycle , Enumerate , Filter , FilterMap , FlatMap , Fuse , Inspect , Map , Peekable , Rev , Scan , Skip , SkipWhile , Take , TakeWhile , Zip , } ;}
mkuse!{# [unstable (feature = "iter_intersperse" , reason = "recently added" , issue = "79524")] pub use self :: adapters :: { Intersperse , IntersperseWith } ;}
mkuse!{# [unstable (feature = "step_trait" , reason = "likely to be replaced by finer-grained traits" , issue = "42168")] pub use self :: range :: Step ;}
mkuse!{# [unstable (feature = "iter_macro" , issue = "142269" , reason = "generators are unstable")] pub use self :: sources :: iter ;}
mkuse!{# [stable (feature = "iter_empty" , since = "1.2.0")] pub use self :: sources :: { Empty , empty } ;}
mkuse!{# [unstable (feature = "iter_from_coroutine" , issue = "43122" , reason = "coroutines are unstable")] pub use self :: sources :: { FromCoroutine , from_coroutine } ;}
mkuse!{# [stable (feature = "iter_from_fn" , since = "1.34.0")] pub use self :: sources :: { FromFn , from_fn } ;}
mkuse!{# [stable (feature = "iter_once" , since = "1.2.0")] pub use self :: sources :: { Once , once } ;}
mkuse!{# [stable (feature = "iter_once_with" , since = "1.43.0")] pub use self :: sources :: { OnceWith , once_with } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: sources :: { Repeat , repeat } ;}
mkuse!{# [stable (feature = "iter_repeat_n" , since = "1.82.0")] pub use self :: sources :: { RepeatN , repeat_n } ;}
mkuse!{# [stable (feature = "iterator_repeat_with" , since = "1.28.0")] pub use self :: sources :: { RepeatWith , repeat_with } ;}
mkuse!{# [stable (feature = "iter_successors" , since = "1.34.0")] pub use self :: sources :: { Successors , successors } ;}
mkuse!{# [stable (feature = "fused" , since = "1.26.0")] pub use self :: traits :: FusedIterator ;}
mkuse!{# [unstable (issue = "none" , feature = "inplace_iteration")] pub use self :: traits :: InPlaceIterable ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: traits :: Iterator ;}
mkuse!{# [unstable (issue = "none" , feature = "trusted_fused")] pub use self :: traits :: TrustedFused ;}
mkuse!{# [unstable (feature = "trusted_len" , issue = "37572")] pub use self :: traits :: TrustedLen ;}
mkuse!{# [unstable (feature = "trusted_step" , issue = "85731")] pub use self :: traits :: TrustedStep ;}
mkuse!{pub (crate) use self :: traits :: UncheckedIterator ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: traits :: { DoubleEndedIterator , ExactSizeIterator , Extend , FromIterator , IntoIterator , Product , Sum , } ;}
mkmod!{adapters, { 
                getname!(adapters);
                getsrc!(adapters);
                getpath!(adapters);
                get_deps!(adapters);
                get_crates!(adapters);
                mkinclude!(adapters);
                 
            }}
mkmod!{range, { 
                getname!(range);
                getsrc!(range);
                getpath!(range);
                get_deps!(range);
                get_crates!(range);
                mkinclude!(range);
                 
            }}
mkmod!{sources, { 
                getname!(sources);
                getsrc!(sources);
                getpath!(sources);
                get_deps!(sources);
                get_crates!(sources);
                mkinclude!(sources);
                 
            }}
mkmod!{traits, { 
                getname!(traits);
                getsrc!(traits);
                getpath!(traits);
                get_deps!(traits);
                get_crates!(traits);
                mkinclude!(traits);
                 
            }}