macro_rules! ClosureCapture {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub struct ClosureCapture < 'db > { owner : DefWithBodyId , closure : InternedClosureId , capture : hir_ty :: CapturedItem < 'db > , }
    };
}

ClosureCapture!()