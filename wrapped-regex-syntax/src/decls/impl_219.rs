macro_rules! deps {
    () => {
        Hir!();
        HirKind!();
        Properties!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        # [doc = " Methods for accessing the underlying `HirKind` and `Properties`."] impl Hir { # [doc = " Returns a reference to the underlying HIR kind."] pub fn kind (& self) -> & HirKind { & self . kind } # [doc = " Consumes ownership of this HIR expression and returns its underlying"] # [doc = " `HirKind`."] pub fn into_kind (mut self) -> HirKind { core :: mem :: replace (& mut self . kind , HirKind :: Empty) } # [doc = " Returns the properties computed for this `Hir`."] pub fn properties (& self) -> & Properties { & self . props } # [doc = " Splits this HIR into its constituent parts."] # [doc = ""] # [doc = " This is useful because `let Hir { kind, props } = hir;` does not work"] # [doc = " because of `Hir`'s custom `Drop` implementation."] fn into_parts (mut self) -> (HirKind , Properties) { (core :: mem :: replace (& mut self . kind , HirKind :: Empty) , core :: mem :: replace (& mut self . props , Properties :: empty ()) ,) } }
    };
}

impl_219!();