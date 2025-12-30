// Generated macro for SliceStream (struct)
macro_rules! Depcrate_streamSliceStream {
() => {
// Module: crate::stream
// Provides: {"SliceStream"}
// Dependencies: {}
# [doc = " Newtype for constructing a stream from a slice where the items in the slice are not copyable."] # [derive (Copy , Eq , PartialEq , Ord , PartialOrd , Debug)] pub struct SliceStream < 'a , T > (pub & 'a [T]) ;
};
}
