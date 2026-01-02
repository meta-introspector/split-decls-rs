mkmod!{borrowed_locals, { 
                getname!(borrowed_locals);
                getsrc!(borrowed_locals);
                getpath!(borrowed_locals);
                get_deps!(borrowed_locals);
                get_crates!(borrowed_locals);
                mkinclude!(borrowed_locals);
                 
            }}
mkmod!{initialized, { 
                getname!(initialized);
                getsrc!(initialized);
                getpath!(initialized);
                get_deps!(initialized);
                get_crates!(initialized);
                mkinclude!(initialized);
                 
            }}
mkmod!{liveness, { 
                getname!(liveness);
                getsrc!(liveness);
                getpath!(liveness);
                get_deps!(liveness);
                get_crates!(liveness);
                mkinclude!(liveness);
                 
            }}
mkmod!{storage_liveness, { 
                getname!(storage_liveness);
                getsrc!(storage_liveness);
                getpath!(storage_liveness);
                get_deps!(storage_liveness);
                get_crates!(storage_liveness);
                mkinclude!(storage_liveness);
                 
            }}
mkuse!{pub use self :: borrowed_locals :: { MaybeBorrowedLocals , borrowed_locals } ;}
mkuse!{pub use self :: initialized :: { EverInitializedPlaces , EverInitializedPlacesDomain , MaybeInitializedPlaces , MaybeUninitializedPlaces , MaybeUninitializedPlacesDomain , } ;}
mkuse!{pub use self :: liveness :: { DefUse , MaybeLiveLocals , MaybeTransitiveLiveLocals , TransferFunction as LivenessTransferFunction , } ;}
mkuse!{pub use self :: storage_liveness :: { MaybeRequiresStorage , MaybeStorageDead , MaybeStorageLive , always_storage_live_locals , } ;}