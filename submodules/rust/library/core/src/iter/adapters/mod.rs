mkuse!{use crate :: iter :: InPlaceIterable ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ops :: { ChangeOutputType , ControlFlow , FromResidual , Residual , Try } ;}
mkmod!{array_chunks, { 
                getname!(array_chunks);
                getsrc!(array_chunks);
                getpath!(array_chunks);
                get_deps!(array_chunks);
                get_crates!(array_chunks);
                mkinclude!(array_chunks);
                 
            }}
mkmod!{by_ref_sized, { 
                getname!(by_ref_sized);
                getsrc!(by_ref_sized);
                getpath!(by_ref_sized);
                get_deps!(by_ref_sized);
                get_crates!(by_ref_sized);
                mkinclude!(by_ref_sized);
                 
            }}
mkmod!{chain, { 
                getname!(chain);
                getsrc!(chain);
                getpath!(chain);
                get_deps!(chain);
                get_crates!(chain);
                mkinclude!(chain);
                 
            }}
mkmod!{cloned, { 
                getname!(cloned);
                getsrc!(cloned);
                getpath!(cloned);
                get_deps!(cloned);
                get_crates!(cloned);
                mkinclude!(cloned);
                 
            }}
mkmod!{copied, { 
                getname!(copied);
                getsrc!(copied);
                getpath!(copied);
                get_deps!(copied);
                get_crates!(copied);
                mkinclude!(copied);
                 
            }}
mkmod!{cycle, { 
                getname!(cycle);
                getsrc!(cycle);
                getpath!(cycle);
                get_deps!(cycle);
                get_crates!(cycle);
                mkinclude!(cycle);
                 
            }}
mkmod!{enumerate, { 
                getname!(enumerate);
                getsrc!(enumerate);
                getpath!(enumerate);
                get_deps!(enumerate);
                get_crates!(enumerate);
                mkinclude!(enumerate);
                 
            }}
mkmod!{filter, { 
                getname!(filter);
                getsrc!(filter);
                getpath!(filter);
                get_deps!(filter);
                get_crates!(filter);
                mkinclude!(filter);
                 
            }}
mkmod!{filter_map, { 
                getname!(filter_map);
                getsrc!(filter_map);
                getpath!(filter_map);
                get_deps!(filter_map);
                get_crates!(filter_map);
                mkinclude!(filter_map);
                 
            }}
mkmod!{flatten, { 
                getname!(flatten);
                getsrc!(flatten);
                getpath!(flatten);
                get_deps!(flatten);
                get_crates!(flatten);
                mkinclude!(flatten);
                 
            }}
mkmod!{fuse, { 
                getname!(fuse);
                getsrc!(fuse);
                getpath!(fuse);
                get_deps!(fuse);
                get_crates!(fuse);
                mkinclude!(fuse);
                 
            }}
mkmod!{inspect, { 
                getname!(inspect);
                getsrc!(inspect);
                getpath!(inspect);
                get_deps!(inspect);
                get_crates!(inspect);
                mkinclude!(inspect);
                 
            }}
mkmod!{intersperse, { 
                getname!(intersperse);
                getsrc!(intersperse);
                getpath!(intersperse);
                get_deps!(intersperse);
                get_crates!(intersperse);
                mkinclude!(intersperse);
                 
            }}
mkmod!{map, { 
                getname!(map);
                getsrc!(map);
                getpath!(map);
                get_deps!(map);
                get_crates!(map);
                mkinclude!(map);
                 
            }}
mkmod!{map_while, { 
                getname!(map_while);
                getsrc!(map_while);
                getpath!(map_while);
                get_deps!(map_while);
                get_crates!(map_while);
                mkinclude!(map_while);
                 
            }}
mkmod!{map_windows, { 
                getname!(map_windows);
                getsrc!(map_windows);
                getpath!(map_windows);
                get_deps!(map_windows);
                get_crates!(map_windows);
                mkinclude!(map_windows);
                 
            }}
mkmod!{peekable, { 
                getname!(peekable);
                getsrc!(peekable);
                getpath!(peekable);
                get_deps!(peekable);
                get_crates!(peekable);
                mkinclude!(peekable);
                 
            }}
mkmod!{rev, { 
                getname!(rev);
                getsrc!(rev);
                getpath!(rev);
                get_deps!(rev);
                get_crates!(rev);
                mkinclude!(rev);
                 
            }}
mkmod!{scan, { 
                getname!(scan);
                getsrc!(scan);
                getpath!(scan);
                get_deps!(scan);
                get_crates!(scan);
                mkinclude!(scan);
                 
            }}
mkmod!{skip, { 
                getname!(skip);
                getsrc!(skip);
                getpath!(skip);
                get_deps!(skip);
                get_crates!(skip);
                mkinclude!(skip);
                 
            }}
mkmod!{skip_while, { 
                getname!(skip_while);
                getsrc!(skip_while);
                getpath!(skip_while);
                get_deps!(skip_while);
                get_crates!(skip_while);
                mkinclude!(skip_while);
                 
            }}
mkmod!{step_by, { 
                getname!(step_by);
                getsrc!(step_by);
                getpath!(step_by);
                get_deps!(step_by);
                get_crates!(step_by);
                mkinclude!(step_by);
                 
            }}
mkmod!{take, { 
                getname!(take);
                getsrc!(take);
                getpath!(take);
                get_deps!(take);
                get_crates!(take);
                mkinclude!(take);
                 
            }}
mkmod!{take_while, { 
                getname!(take_while);
                getsrc!(take_while);
                getpath!(take_while);
                get_deps!(take_while);
                get_crates!(take_while);
                mkinclude!(take_while);
                 
            }}
mkmod!{zip, { 
                getname!(zip);
                getsrc!(zip);
                getpath!(zip);
                get_deps!(zip);
                get_crates!(zip);
                mkinclude!(zip);
                 
            }}
mkuse!{# [unstable (feature = "iter_array_chunks" , reason = "recently added" , issue = "100450")] pub use self :: array_chunks :: ArrayChunks ;}
mkuse!{# [unstable (feature = "std_internals" , issue = "none")] pub use self :: by_ref_sized :: ByRefSized ;}
mkuse!{# [stable (feature = "iter_chain" , since = "1.91.0")] pub use self :: chain :: chain ;}
mkuse!{# [stable (feature = "iter_cloned" , since = "1.1.0")] pub use self :: cloned :: Cloned ;}
mkuse!{# [stable (feature = "iter_copied" , since = "1.36.0")] pub use self :: copied :: Copied ;}
mkuse!{# [stable (feature = "iterator_flatten" , since = "1.29.0")] pub use self :: flatten :: Flatten ;}
mkuse!{# [unstable (feature = "iter_intersperse" , reason = "recently added" , issue = "79524")] pub use self :: intersperse :: { Intersperse , IntersperseWith } ;}
mkuse!{# [stable (feature = "iter_map_while" , since = "1.57.0")] pub use self :: map_while :: MapWhile ;}
mkuse!{# [unstable (feature = "iter_map_windows" , reason = "recently added" , issue = "87155")] pub use self :: map_windows :: MapWindows ;}
mkuse!{# [stable (feature = "iterator_step_by" , since = "1.28.0")] pub use self :: step_by :: StepBy ;}
mkuse!{# [unstable (feature = "trusted_random_access" , issue = "none")] pub use self :: zip :: TrustedRandomAccess ;}
mkuse!{# [unstable (feature = "trusted_random_access" , issue = "none")] pub use self :: zip :: TrustedRandomAccessNoCoerce ;}
mkuse!{# [stable (feature = "iter_zip" , since = "1.59.0")] pub use self :: zip :: zip ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: { chain :: Chain , cycle :: Cycle , enumerate :: Enumerate , filter :: Filter , filter_map :: FilterMap , flatten :: FlatMap , fuse :: Fuse , inspect :: Inspect , map :: Map , peekable :: Peekable , rev :: Rev , scan :: Scan , skip :: Skip , skip_while :: SkipWhile , take :: Take , take_while :: TakeWhile , zip :: Zip , } ;}
mkitem!{mktrait!{# [doc = " This trait provides transitive access to source-stage in an iterator-adapter pipeline"] # [doc = " under the conditions that"] # [doc = " * the iterator source `S` itself implements `SourceIter<Source = S>`"] # [doc = " * there is a delegating implementation of this trait for each adapter in the pipeline between"] # [doc = "   the source and the pipeline consumer."] # [doc = ""] # [doc = " When the source is an owning iterator struct (commonly called `IntoIter`) then"] # [doc = " this can be useful for specializing [`FromIterator`] implementations or recovering the"] # [doc = " remaining elements after an iterator has been partially exhausted."] # [doc = ""] # [doc = " Note that implementations do not necessarily have to provide access to the innermost"] # [doc = " source of a pipeline. A stateful intermediate adapter might eagerly evaluate a part"] # [doc = " of the pipeline and expose its internal storage as source."] # [doc = ""] # [doc = " The trait is unsafe because implementers must uphold additional safety properties."] # [doc = " See [`as_inner`] for details."] # [doc = ""] # [doc = " The primary use of this trait is in-place iteration. Refer to the [`vec::in_place_collect`]"] # [doc = " module documentation for more information."] # [doc = ""] # [doc = " [`vec::in_place_collect`]: ../../../../alloc/vec/in_place_collect/index.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Retrieving a partially consumed source:"] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(inplace_iteration)]"] # [doc = " # use std::iter::SourceIter;"] # [doc = ""] # [doc = " let mut iter = vec![9, 9, 9].into_iter().map(|i| i * i);"] # [doc = " let _ = iter.next();"] # [doc = " let mut remainder = std::mem::replace(unsafe { iter.as_inner() }, Vec::new().into_iter());"] # [doc = " println!(\"n = {} elements remaining\", remainder.len());"] # [doc = " ```"] # [doc = ""] # [doc = " [`FromIterator`]: crate::iter::FromIterator"] # [doc = " [`as_inner`]: SourceIter::as_inner"] # [unstable (issue = "none" , feature = "inplace_iteration")] # [doc (hidden)] # [rustc_specialization_trait] pub unsafe trait SourceIter { # [doc = " A source stage in an iterator pipeline."] type Source ; # [doc = " Retrieve the source of an iterator pipeline."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations must return the same mutable reference for their lifetime, unless"] # [doc = " replaced by a caller."] # [doc = ""] # [doc = " Callers may only replace the reference when they stopped iteration and drop the"] # [doc = " iterator pipeline after extracting the source."] # [doc = ""] # [doc = " This means iterator adapters can rely on the source not changing during"] # [doc = " iteration but they cannot rely on it in their Drop implementations."] # [doc = ""] # [doc = " Implementing this method means adapters relinquish private-only access to their"] # [doc = " source and can only rely on guarantees made based on method receiver types."] # [doc = " The lack of restricted access also requires that adapters must uphold the source's"] # [doc = " public API even when they have access to its internals."] # [doc = ""] # [doc = " Callers in turn must expect the source to be in any state that is consistent with"] # [doc = " its public API since adapters sitting between it and the source have the same"] # [doc = " access. In particular an adapter may have consumed more elements than strictly necessary."] # [doc = ""] # [doc = " The overall goal of these requirements is to let the consumer of a pipeline use"] # [doc = " * whatever remains in the source after iteration has stopped"] # [doc = " * the memory that has become unused by advancing a consuming iterator"] # [doc = ""] # [doc = " [`next()`]: Iterator::next()"] unsafe fn as_inner (& mut self) -> & mut Self :: Source ; }}}
mkitem!{mkstruct!{# [doc = " An iterator adapter that produces output as long as the underlying"] # [doc = " iterator produces values where `Try::branch` says to `ControlFlow::Continue`."] # [doc = ""] # [doc = " If a `ControlFlow::Break` is encountered, the iterator stops and the"] # [doc = " residual is stored."] pub (crate) struct GenericShunt < 'a , I , R > { iter : I , residual : & 'a mut Option < R > , }}}

macro_rules! try_process_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_process in module {}", module_path!());
    };
}

mkfn!{
    try_process_introspect!();
    # [doc = " Process the given iterator as if it yielded the item's `Try::Output`"] # [doc = " type instead. Any `Try::Residual`s encountered will stop the inner iterator"] # [doc = " and be propagated back to the overall result."] pub (crate) fn try_process < I , T , R , F , U > (iter : I , mut f : F) -> ChangeOutputType < I :: Item , U > where I : Iterator < Item : Try < Output = T , Residual = R > > , for < 'a > F : FnMut (GenericShunt < 'a , I , R >) -> U , R : Residual < U > , { let mut residual = None ; let shunt = GenericShunt { iter , residual : & mut residual } ; let value = f (shunt) ; match residual { Some (r) => FromResidual :: from_residual (r) , None => Try :: from_output (value) , } }
}
mkitem!{mkimpl!{impl < I , R > Iterator for GenericShunt < '_ , I , R > where I : Iterator < Item : Try < Residual = R > > , { type Item = < I :: Item as Try > :: Output ; fn next (& mut self) -> Option < Self :: Item > { self . try_for_each (ControlFlow :: Break) . break_value () } fn size_hint (& self) -> (usize , Option < usize >) { if self . residual . is_some () { (0 , Some (0)) } else { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } } fn try_fold < B , F , T > (& mut self , init : B , mut f : F) -> T where F : FnMut (B , Self :: Item) -> T , T : Try < Output = B > , { self . iter . try_fold (init , | acc , x | match Try :: branch (x) { ControlFlow :: Continue (x) => ControlFlow :: from_try (f (acc , x)) , ControlFlow :: Break (r) => { * self . residual = Some (r) ; ControlFlow :: Break (try { acc }) } }) . into_try () } impl_fold_via_try_fold ! { fold -> try_fold } }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I , R > SourceIter for GenericShunt < '_ , I , R > where I : SourceIter , { type Source = I :: Source ; # [inline] unsafe fn as_inner (& mut self) -> & mut Self :: Source { unsafe { SourceIter :: as_inner (& mut self . iter) } } }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I , R > InPlaceIterable for GenericShunt < '_ , I , R > where I : InPlaceIterable , { const EXPAND_BY : Option < NonZero < usize > > = I :: EXPAND_BY ; const MERGE_BY : Option < NonZero < usize > > = I :: MERGE_BY ; }}}