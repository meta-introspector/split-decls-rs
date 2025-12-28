macro_rules! BorrowedContentSource {
    () => {
        pub (super) enum BorrowedContentSource < 'tcx > { DerefRawPointer , DerefMutableRef , DerefSharedRef , OverloadedDeref (Ty < 'tcx >) , OverloadedIndex (Ty < 'tcx >) , }
    };
}

BorrowedContentSource!();