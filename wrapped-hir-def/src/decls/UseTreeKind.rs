macro_rules! deps {
    () => {
        UseTree!();
        Item!();
        ImportAlias!();
    };
}

macro_rules! UseTreeKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub enum UseTreeKind { # [doc = " ```ignore"] # [doc = " use path::to::Item;"] # [doc = " use path::to::Item as Renamed;"] # [doc = " use path::to::Trait as _;"] # [doc = " ```"] Single { path : Interned < ModPath > , alias : Option < ImportAlias > } , # [doc = " ```ignore"] # [doc = " use *;  // (invalid, but can occur in nested tree)"] # [doc = " use path::*;"] # [doc = " ```"] Glob { path : Option < Interned < ModPath > > } , # [doc = " ```ignore"] # [doc = " use prefix::{self, Item, ...};"] # [doc = " ```"] Prefixed { prefix : Option < Interned < ModPath > > , list : Box < [UseTree] > } , }
    };
}

UseTreeKind!();