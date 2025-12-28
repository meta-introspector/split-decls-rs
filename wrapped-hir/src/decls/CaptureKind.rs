macro_rules! CaptureKind {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq)] pub enum CaptureKind { SharedRef , UniqueSharedRef , MutableRef , Move , }
    };
}

CaptureKind!()