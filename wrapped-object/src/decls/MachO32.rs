macro_rules! MachO32 {
    () => {
        struct MachO32 < E > { endian : E , }
    };
}

MachO32!();