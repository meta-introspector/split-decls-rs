macro_rules! deps {
    () => {
        ConflictMapping!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl ConflictMapping { fn is_swapped (& self) -> bool { matches ! (self , ConflictMapping :: Swapped) } fn swapped (self) -> ConflictMapping { match self { ConflictMapping :: Original => ConflictMapping :: Swapped , ConflictMapping :: Swapped => ConflictMapping :: Original , } } fn to_global (self , global : ConflictMapping) -> ConflictMapping { match global { ConflictMapping :: Original => self , ConflictMapping :: Swapped => self . swapped () , } } }
    };
}

impl_105!()