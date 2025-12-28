macro_rules! deps {
    () => {
        Finder!();
        BStr!();
        BString!();
        FinderReverse!();
    };
}

macro_rules! apitests {
    () => {
        deps!();
        # [cfg (all (test , feature = "std"))] mod apitests { use crate :: { bstr :: BStr , bstring :: BString , ext_slice :: { Finder , FinderReverse } , } ; # [test] fn oibits () { use std :: panic :: { RefUnwindSafe , UnwindSafe } ; fn assert_send < T : Send > () { } fn assert_sync < T : Sync > () { } fn assert_unwind_safe < T : RefUnwindSafe + UnwindSafe > () { } assert_send :: < & BStr > () ; assert_sync :: < & BStr > () ; assert_unwind_safe :: < & BStr > () ; assert_send :: < BString > () ; assert_sync :: < BString > () ; assert_unwind_safe :: < BString > () ; assert_send :: < Finder < '_ > > () ; assert_sync :: < Finder < '_ > > () ; assert_unwind_safe :: < Finder < '_ > > () ; assert_send :: < FinderReverse < '_ > > () ; assert_sync :: < FinderReverse < '_ > > () ; assert_unwind_safe :: < FinderReverse < '_ > > () ; } }
    };
}

apitests!()