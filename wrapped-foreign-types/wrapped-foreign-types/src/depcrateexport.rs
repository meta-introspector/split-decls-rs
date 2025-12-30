// Generated macro for export (module)
macro_rules! Depcrateexport {
() => {
// Module: crate
// Provides: {"export"}
// Dependencies: {}
# [doc (hidden)] pub mod export { pub use core :: borrow :: { Borrow , BorrowMut } ; pub use core :: clone :: Clone ; pub use core :: convert :: { AsMut , AsRef } ; pub use core :: marker :: { PhantomData , Send , Sync } ; pub use core :: ops :: { Deref , DerefMut , Drop } ; pub use core :: ptr :: NonNull ; # [cfg (feature = "std")] pub use std :: borrow :: ToOwned ; }
};
}
