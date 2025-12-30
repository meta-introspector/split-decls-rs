// Generated macro for Arena (struct)
macro_rules! DepcrateArena {
() => {
// Module: crate
// Provides: {"Arena"}
// Dependencies: {}
# [doc = " An arena of objects of type `T`."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use typed_arena::Arena;"] # [doc = ""] # [doc = " struct Monster {"] # [doc = "     level: u32,"] # [doc = " }"] # [doc = ""] # [doc = " let monsters = Arena::new();"] # [doc = ""] # [doc = " let vegeta = monsters.alloc(Monster { level: 9001 });"] # [doc = " assert!(vegeta.level > 9000);"] # [doc = " ```"] pub struct Arena < T > { chunks : RefCell < ChunkList < T > > , }
};
}
