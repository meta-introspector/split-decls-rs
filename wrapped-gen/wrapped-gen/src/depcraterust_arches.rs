// Generated macro for rust_arches (function)
macro_rules! Depcraterust_arches {
() => {
// Module: crate
// Provides: {"rust_arches"}
// Dependencies: {}
fn rust_arches (linux_arch : & str) -> & [& str] { match linux_arch { "arm" => & ["arm"] , "arm64" => & ["aarch64"] , "avr32" => & ["avr"] , "csky" => & ["csky"] , "hexagon" => & ["hexagon"] , "loongarch" => & ["loongarch64"] , "mips" => & ["mips" , "mips64" , "mips32r6" , "mips64r6"] , "powerpc" => & ["powerpc" , "powerpc64"] , "riscv" => & ["riscv32" , "riscv64"] , "s390" => & ["s390x"] , "sparc" => & ["sparc" , "sparc64"] , "x86" => & ["x86" , "x86_64" , "x32"] , "alpha" | "cris" | "h8300" | "m68k" | "microblaze" | "mn10300" | "score" | "blackfin" | "frv" | "ia64" | "m32r" | "m68knommu" | "parisc" | "sh" | "um" | "xtensa" | "unicore32" | "c6x" | "nios2" | "openrisc" | "arc" | "nds32" | "metag" | "tile" => & [] , _ => panic ! ("unrecognized arch: {}" , linux_arch) , } }
};
}
