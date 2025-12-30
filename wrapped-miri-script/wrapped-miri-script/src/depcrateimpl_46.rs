// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl Command { fn add_remainder (& mut self , remainder : Vec < String >) -> Result < () > { if remainder . is_empty () { return Ok (()) ; } match self { Self :: Install { flags , .. } | Self :: Build { flags , .. } | Self :: Check { flags , .. } | Self :: Doc { flags , .. } | Self :: Fmt { flags } | Self :: Toolchain { flags } | Self :: Clippy { flags , .. } | Self :: Run { flags , .. } | Self :: Test { flags , .. } => { flags . extend (remainder) ; Ok (()) } Self :: Bench { .. } | Self :: Squash => bail ! ("unexpected \"--\" found in arguments") , } } }
};
}
