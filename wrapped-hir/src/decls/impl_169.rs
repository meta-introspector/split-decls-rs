macro_rules! deps {
    () => {
        ClosureCapture!();
        CaptureKind!();
        CaptureUsages!();
        Local!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < 'db > ClosureCapture < 'db > { pub fn local (& self) -> Local { Local { parent : self . owner , binding_id : self . capture . local () } } # [doc = " Returns whether this place has any field (aka. non-deref) projections."] pub fn has_field_projections (& self) -> bool { self . capture . has_field_projections () } pub fn usages (& self) -> CaptureUsages { CaptureUsages { parent : self . owner , spans : self . capture . spans () } } pub fn kind (& self) -> CaptureKind { match self . capture . kind () { hir_ty :: CaptureKind :: ByRef (hir_ty :: mir :: BorrowKind :: Shallow | hir_ty :: mir :: BorrowKind :: Shared ,) => CaptureKind :: SharedRef , hir_ty :: CaptureKind :: ByRef (hir_ty :: mir :: BorrowKind :: Mut { kind : MutBorrowKind :: ClosureCapture , }) => CaptureKind :: UniqueSharedRef , hir_ty :: CaptureKind :: ByRef (hir_ty :: mir :: BorrowKind :: Mut { kind : MutBorrowKind :: Default | MutBorrowKind :: TwoPhasedBorrow , }) => CaptureKind :: MutableRef , hir_ty :: CaptureKind :: ByValue => CaptureKind :: Move , } } # [doc = " Converts the place to a name that can be inserted into source code."] pub fn place_to_name (& self , db : & dyn HirDatabase) -> String { self . capture . place_to_name (self . owner , db) } pub fn display_place_source_code (& self , db : & dyn HirDatabase) -> String { self . capture . display_place_source_code (self . owner , db) } pub fn display_place (& self , db : & dyn HirDatabase) -> String { self . capture . display_place (self . owner , db) } }
    };
}

impl_169!()