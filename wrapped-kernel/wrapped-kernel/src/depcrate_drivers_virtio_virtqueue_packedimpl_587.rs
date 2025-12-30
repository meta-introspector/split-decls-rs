// Generated macro for impl_587 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedimpl_587 {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"impl_587"}
// Dependencies: {}
impl ReadCtrl < '_ > { # [doc = " Polls the ring for a new finished buffer. If buffer is marked as finished, takes care of"] # [doc = " updating the queue and returns the respective TransferToken."] fn poll_next (& mut self) -> Option < (TransferToken < pvirtq :: Desc > , u32) > { let desc = & self . desc_ring . ring [usize :: from (self . position)] ; if self . desc_ring . is_marked_used (desc . flags) { let buff_id = desc . id . to_ne () ; let tkn = self . desc_ring . tkn_ref_ring [usize :: from (buff_id)] . take () . expect ("The buff_id is incorrect or the reference to the TransferToken was misplaced." ,) ; let write_len = desc . len . to_ne () ; for _ in 0 .. tkn . num_consuming_descr () { self . incrmt () ; } self . desc_ring . mem_pool . ret_id (buff_id) ; Some ((tkn , write_len)) } else { None } } fn incrmt (& mut self) { if self . desc_ring . poll_index + 1 == self . modulo { self . desc_ring . dev_wc ^= true ; } assert ! (self . desc_ring . capacity <= u16 :: try_from (self . desc_ring . ring . len ()) . unwrap ()) ; self . desc_ring . capacity += 1 ; self . desc_ring . poll_index = (self . desc_ring . poll_index + 1) % self . modulo ; self . position = self . desc_ring . poll_index ; } }
};
}
