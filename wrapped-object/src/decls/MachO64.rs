macro_rules! MachO64 {
    () => {
        struct MachO64 < E > { endian : E , }
    };
}

MachO64!();