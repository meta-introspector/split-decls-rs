macro_rules! probes_from_flags {
    () => {
        const fn probes_from_flags (flags : u32) -> [u32 ; 2] { [1 + ((flags & 0xFFF) + 2) / 3 , 1 + (((flags & 0xFFF) >> 2) + 2) / 3 ,] }
    };
}

probes_from_flags!()