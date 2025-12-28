macro_rules! buffer_select {
    () => {
        # [doc = " Return which dynamic buffer was used by this operation."] # [doc = ""] # [doc = " This corresponds to the `IORING_CQE_F_BUFFER` flag (and related bit-shifting),"] # [doc = " and it signals to the consumer which provided contains the result of this"] # [doc = " operation."] pub fn buffer_select (flags : u32) -> Option < u16 > { if flags & sys :: IORING_CQE_F_BUFFER != 0 { let id = flags >> sys :: IORING_CQE_BUFFER_SHIFT ; Some (id as u16) } else { None } }
    };
}

buffer_select!();