macro_rules! deps {
    () => {
        Generation!();
        StateId!();
    };
}

macro_rules! SlotIndexMarker {
    () => {
        deps!();
        # [doc = " A way to indicate which pack indices we have seen already and which of them are loaded, along with an idea"] # [doc = " of whether stored `PackId`s are still usable."] # [derive (Default , Copy , Clone , Debug)] pub struct SlotIndexMarker { # [doc = " The generation the `loaded_until_index` belongs to. Indices of different generations are completely incompatible."] # [doc = " This value changes once the internal representation is compacted, something that may happen only if there is no handle"] # [doc = " requiring stable pack indices."] pub (crate) generation : Generation , # [doc = " A unique id identifying the index state as well as all loose databases we have last observed."] # [doc = " If it changes in any way, the value is different."] pub (crate) state_id : StateId , }
    };
}

SlotIndexMarker!()