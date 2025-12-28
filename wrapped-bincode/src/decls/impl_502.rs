macro_rules! deps {
    () => {
        EncodeError!();
    };
}

macro_rules! impl_502 {
    () => {
        deps!();
        impl core :: error :: Error for EncodeError { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: RefCellAlreadyBorrowed { inner , .. } => Some (inner) , # [cfg (feature = "std")] Self :: Io { inner , .. } => Some (inner) , # [cfg (feature = "std")] Self :: InvalidSystemTime { inner , .. } => Some (inner) , _ => None , } } }
    };
}

impl_502!();