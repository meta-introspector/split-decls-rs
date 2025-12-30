// Generated macro for impl_427 (impl)
macro_rules! Depcrate_drivers_net_virtioimpl_427 {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"impl_427"}
// Dependencies: {}
impl smoltcp :: phy :: TxToken for TxToken < '_ > { fn consume < R , F > (self , len : usize , f : F) -> R where F : FnOnce (& mut [u8]) -> R , { let mut token = ManuallyDrop :: new (self) ; assert ! (len <= usize :: try_from (token . send_vqs . buf_size) . unwrap ()) ; let mut packet = Vec :: with_capacity_in (len , DeviceAlloc) ; let result = unsafe { let result = f (packet . spare_capacity_mut () . assume_init_mut ()) ; packet . set_len (len) ; result } ; let mut header = Box :: new_in (< Hdr as Default > :: default () , DeviceAlloc) ; if let Some ((ip_header_len , csum_offset)) = VirtioNetDriver :: should_request_checksum (& token . checksums , & mut packet) { header . flags = HdrF :: NEEDS_CSUM ; header . csum_start = (u16 :: try_from (ETHERNET_HEADER_LEN) . unwrap () + ip_header_len) . into () ; header . csum_offset = csum_offset . into () ; } let buff_tkn = AvailBufferToken :: new (SmallVec :: from_buf ([BufferElem :: Sized (header) , BufferElem :: Vector (packet)]) , SmallVec :: new () ,) . unwrap () ; token . send_vqs . vqs [0] . dispatch (buff_tkn , false , BufferType :: Direct) . unwrap () ; result } }
};
}
