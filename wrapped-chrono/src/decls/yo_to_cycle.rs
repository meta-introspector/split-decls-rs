macro_rules! yo_to_cycle {
    () => {
        const fn yo_to_cycle (year_mod_400 : u32 , ordinal : u32) -> u32 { year_mod_400 * 365 + YEAR_DELTAS [year_mod_400 as usize] as u32 + ordinal - 1 }
    };
}

yo_to_cycle!();