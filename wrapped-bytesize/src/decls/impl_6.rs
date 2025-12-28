macro_rules! deps {
    () => {
        Format!();
        Display!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Display { # [doc = " Format using IEC (binary) units."] # [doc = ""] # [doc = " E.g., `11.8 MiB`."] # [must_use] # [doc (alias = "binary")] pub fn iec (mut self) -> Self { self . format = Format :: Iec ; self } # [doc = " Format using a short style and IEC (binary) units."] # [doc = ""] # [doc = " E.g., `11.8M`."] # [doc = ""] # [doc = " Designed to produce output compatible with `sort -h`."] # [must_use] # [doc (alias = "binary")] pub fn iec_short (mut self) -> Self { self . format = Format :: IecShort ; self } # [doc = " Format using SI (decimal) units."] # [doc = ""] # [doc = " E.g., `12.3 MB`."] # [must_use] # [doc (alias = "decimal")] pub fn si (mut self) -> Self { self . format = Format :: Si ; self } # [doc = " Format using a short style and SI (decimal) units."] # [doc = ""] # [doc = " E.g., `12.3M`."] # [must_use] # [doc (alias = "decimal")] pub fn si_short (mut self) -> Self { self . format = Format :: SiShort ; self } }
    };
}

impl_6!();