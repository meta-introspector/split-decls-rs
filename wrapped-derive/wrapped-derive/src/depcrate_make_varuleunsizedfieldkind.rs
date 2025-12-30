// Generated macro for UnsizedFieldKind (enum)
macro_rules! Depcrate_make_varuleUnsizedFieldKind {
() => {
// Module: crate::make_varule
// Provides: {"UnsizedFieldKind"}
// Dependencies: {}
# [doc = " Represents the type of the last field of the struct"] # [derive (Clone , Debug)] enum UnsizedFieldKind < 'a > { Cow (OwnULETy < 'a >) , VarZeroCow (OwnULETy < 'a >) , ZeroVec (& 'a Type) , VarZeroVec (& 'a Type) , # [doc = " Custom VarULE type, and the identifier corresponding to the VarULE type"] Custom (& 'a TypePath , Ident) , Growable (OwnULETy < 'a >) , Boxed (OwnULETy < 'a >) , Ref (OwnULETy < 'a >) , }
};
}
