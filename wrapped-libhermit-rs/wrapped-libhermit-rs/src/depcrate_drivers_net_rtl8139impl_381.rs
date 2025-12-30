// Generated macro for impl_381 (impl)
macro_rules! Depcrate_drivers_net_rtl8139impl_381 {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"impl_381"}
// Dependencies: {}
impl < 'a > smoltcp :: phy :: TxToken for TxToken < 'a > { fn consume < R , F > (self , len : usize , f : F) -> R where F : FnOnce (& mut [u8]) -> R , { let mut token = ManuallyDrop :: new (self) ; let id = token . tx_fields . tx_counter % NO_TX_BUFFERS ; assert ! (! token . tx_fields . tx_in_use [id] && len <= TX_BUF_LEN , "Unable to get TX buffer") ; token . tx_fields . tx_in_use [id] = true ; token . tx_fields . tx_counter += 1 ; let buffer = & mut token . tx_fields . txbuffer [id * TX_BUF_LEN ..] [.. len] ; let result = f (buffer) ; let len = le32 :: from (u32 :: try_from (len) . unwrap ()) ; match id { 0 => token . tsd0 . write (len) , 1 => token . tsd1 . write (len) , 2 => token . tsd2 . write (len) , 3 => token . tsd3 . write (len) , _ => unreachable ! () , } ; result } }
};
}
