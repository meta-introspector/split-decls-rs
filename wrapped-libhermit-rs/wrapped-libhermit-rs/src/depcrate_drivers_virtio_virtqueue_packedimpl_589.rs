// Generated macro for impl_589 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedimpl_589 {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"impl_589"}
// Dependencies: {}
impl WriteCtrl < '_ > { # [doc = " **This function MUST only be used within the WriteCtrl.write_desc() function!**"] # [doc = ""] # [doc = " Incrementing index by one. The index wrappes around to zero when"] # [doc = " reaching (modulo -1)."] # [doc = ""] # [doc = " Also takes care of wrapping the wrap counter of the associated"] # [doc = " DescriptorRing."] fn incrmt (& mut self) { assert ! (self . desc_ring . capacity != 0) ; self . desc_ring . capacity -= 1 ; if self . position + 1 == self . modulo { self . desc_ring . drv_wc ^= true ; } self . desc_ring . write_index = (self . desc_ring . write_index + 1) % self . modulo ; self . position = (self . position + 1) % self . modulo ; } # [doc = " Completes the descriptor flags and id, and writes into the queue at the correct position."] fn write_desc (& mut self , mut incomplete_desc : pvirtq :: Desc) { incomplete_desc . id = self . buff_id . into () ; if self . start == self . position { self . first_flags = self . desc_ring . to_marked_avail (incomplete_desc . flags) ; } else { incomplete_desc . flags = self . desc_ring . to_marked_avail (incomplete_desc . flags) ; } self . desc_ring . ring [usize :: from (self . position)] = incomplete_desc ; self . incrmt () ; } fn make_avail (& mut self , raw_tkn : TransferToken < pvirtq :: Desc >) { assert ! (self . start != self . position) ; self . desc_ring . make_avail_with_state (raw_tkn , self . start , self . buff_id , self . first_flags) ; } }
};
}
