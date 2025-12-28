macro_rules! deps {
    () => {
        BufRingEntry!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        # [doc = " An entry in a buf_ring that allows setting the address, length and buffer id."] # [allow (clippy :: len_without_is_empty)] impl BufRingEntry { # [doc = " Sets the entry addr."] pub fn set_addr (& mut self , addr : u64) { self . 0 . addr = addr ; } # [doc = " Returns the entry addr."] pub fn addr (& self) -> u64 { self . 0 . addr } # [doc = " Sets the entry len."] pub fn set_len (& mut self , len : u32) { self . 0 . len = len ; } # [doc = " Returns the entry len."] pub fn len (& self) -> u32 { self . 0 . len } # [doc = " Sets the entry bid."] pub fn set_bid (& mut self , bid : u16) { self . 0 . bid = bid ; } # [doc = " Returns the entry bid."] pub fn bid (& self) -> u16 { self . 0 . bid } # [doc = " The offset to the ring's tail field given the ring's base address."] # [doc = ""] # [doc = " The caller should ensure the ring's base address is aligned with the system's page size,"] # [doc = " per the uring interface requirements."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The ptr will be dereferenced in order to determine the address of the resv field,"] # [doc = " so the caller is responsible for passing in a valid pointer. And not just"] # [doc = " a valid pointer type, but also the argument must be the address to the first entry"] # [doc = " of the buf_ring for the resv field to even be considered the tail field of the ring."] # [doc = " The entry must also be properly initialized."] pub unsafe fn tail (ring_base : * const BufRingEntry) -> * const u16 { std :: ptr :: addr_of ! ((* ring_base) . 0 . resv) } }
    };
}

impl_192!();