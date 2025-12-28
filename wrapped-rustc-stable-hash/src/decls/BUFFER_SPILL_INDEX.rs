macro_rules! BUFFER_SPILL_INDEX {
    () => {
        const BUFFER_SPILL_INDEX : usize = BUFFER_WITH_SPILL_CAPACITY - 1 ;
    };
}

BUFFER_SPILL_INDEX!()