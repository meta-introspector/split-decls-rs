macro_rules! cpu_relax {
    () => {
        # [inline] fn cpu_relax (iterations : u32) { for _ in 0 .. iterations { spin_loop () } }
    };
}

cpu_relax!();