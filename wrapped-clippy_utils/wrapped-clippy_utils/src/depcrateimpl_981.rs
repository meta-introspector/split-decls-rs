// Generated macro for impl_981 (impl)
macro_rules! Depcrateimpl_981 {
() => {
// Module: crate
// Provides: {"impl_981"}
// Dependencies: {}
impl std :: ops :: BitOr for CaptureKind { type Output = Self ; fn bitor (self , rhs : Self) -> Self :: Output { match (self , rhs) { (CaptureKind :: Value , _) | (_ , CaptureKind :: Value) => CaptureKind :: Value , (CaptureKind :: Use , _) | (_ , CaptureKind :: Use) => CaptureKind :: Use , (CaptureKind :: Ref (Mutability :: Mut) , CaptureKind :: Ref (_)) | (CaptureKind :: Ref (_) , CaptureKind :: Ref (Mutability :: Mut)) => CaptureKind :: Ref (Mutability :: Mut) , (CaptureKind :: Ref (Mutability :: Not) , CaptureKind :: Ref (Mutability :: Not)) => CaptureKind :: Ref (Mutability :: Not) , } } }
};
}
