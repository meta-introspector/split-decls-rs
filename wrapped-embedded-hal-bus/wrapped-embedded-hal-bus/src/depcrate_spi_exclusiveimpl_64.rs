// Generated macro for impl_64 (impl)
macro_rules! Depcrate_spi_exclusiveimpl_64 {
() => {
// Module: crate::spi::exclusive
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (feature = "async")] # [cfg_attr (docsrs , doc (cfg (feature = "async")))] impl < Word : Copy + 'static , BUS , CS , D > AsyncSpiDevice < Word > for ExclusiveDevice < BUS , CS , D > where BUS : AsyncSpiBus < Word > , CS : OutputPin , D : AsyncDelayNs , { # [inline] async fn transaction (& mut self , operations : & mut [Operation < '_ , Word >] ,) -> Result < () , Self :: Error > { self . cs . set_low () . map_err (DeviceError :: Cs) ? ; let op_res = 'ops : { for op in operations { let res = match op { Operation :: Read (buf) => self . bus . read (buf) . await , Operation :: Write (buf) => self . bus . write (buf) . await , Operation :: Transfer (read , write) => self . bus . transfer (read , write) . await , Operation :: TransferInPlace (buf) => self . bus . transfer_in_place (buf) . await , Operation :: DelayNs (ns) => match self . bus . flush () . await { Err (e) => Err (e) , Ok (()) => { self . delay . delay_ns (* ns) . await ; Ok (()) } } , } ; if let Err (e) = res { break 'ops Err (e) ; } } Ok (()) } ; let flush_res = self . bus . flush () . await ; let cs_res = self . cs . set_high () ; op_res . map_err (DeviceError :: Spi) ? ; flush_res . map_err (DeviceError :: Spi) ? ; cs_res . map_err (DeviceError :: Cs) ? ; Ok (()) } }
};
}
