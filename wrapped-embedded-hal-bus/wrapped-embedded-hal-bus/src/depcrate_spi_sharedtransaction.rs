// Generated macro for transaction (function)
macro_rules! Depcrate_spi_sharedtransaction {
() => {
// Module: crate::spi::shared
// Provides: {"transaction"}
// Dependencies: {}
# [doc = " Common implementation to perform a transaction against the device."] # [inline] pub fn transaction < Word , BUS , CS , D > (operations : & mut [Operation < Word >] , bus : & mut BUS , delay : & mut D , cs : & mut CS ,) -> Result < () , DeviceError < BUS :: Error , CS :: Error > > where BUS : SpiBus < Word > + ErrorType , CS : OutputPin , D : DelayNs , Word : Copy , { cs . set_low () . map_err (DeviceError :: Cs) ? ; let op_res = operations . iter_mut () . try_for_each (| op | match op { Operation :: Read (buf) => bus . read (buf) , Operation :: Write (buf) => bus . write (buf) , Operation :: Transfer (read , write) => bus . transfer (read , write) , Operation :: TransferInPlace (buf) => bus . transfer_in_place (buf) , Operation :: DelayNs (ns) => { bus . flush () ? ; delay . delay_ns (* ns) ; Ok (()) } }) ; let flush_res = bus . flush () ; let cs_res = cs . set_high () ; op_res . map_err (DeviceError :: Spi) ? ; flush_res . map_err (DeviceError :: Spi) ? ; cs_res . map_err (DeviceError :: Cs) ? ; Ok (()) }
};
}
