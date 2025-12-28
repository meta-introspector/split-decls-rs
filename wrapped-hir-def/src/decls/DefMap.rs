macro_rules! deps {
    () => {
        MacroId!();
        BlockInfo!();
        ModuleId!();
        ModuleData!();
        Item!();
        DefMapCrateData!();
    };
}

macro_rules! DefMap {
    () => {
        deps!();
        # [doc = " Contains the results of (early) name resolution."] # [doc = ""] # [doc = " A `DefMap` stores the module tree and the definitions that are in scope in every module after"] # [doc = " item-level macros have been expanded."] # [doc = ""] # [doc = " Every crate has a primary `DefMap` whose root is the crate's main file (`main.rs`/`lib.rs`),"] # [doc = " computed by the `crate_def_map` query. Additionally, every block expression introduces the"] # [doc = " opportunity to write arbitrary item and module hierarchies, and thus gets its own `DefMap` that"] # [doc = " is computed by the `block_def_map` query."] # [derive (Debug , PartialEq , Eq)] pub struct DefMap { # [doc = " The crate this `DefMap` belongs to."] krate : Crate , # [doc = " When this is a block def map, this will hold the block id of the block and module that"] # [doc = " contains this block."] block : Option < BlockInfo > , # [doc = " The modules and their data declared in this crate."] pub modules : Arena < ModuleData > , # [doc = " The prelude module for this crate. This either comes from an import"] # [doc = " marked with the `prelude_import` attribute, or (in the normal case) from"] # [doc = " a dependency (`std` or `core`)."] # [doc = " The prelude is empty for non-block DefMaps (unless `#[prelude_import]` was used,"] # [doc = " but that attribute is nightly and when used in a block, it affects resolution globally"] # [doc = " so we aren't handling this correctly anyways)."] prelude : Option < (ModuleId , Option < UseId >) > , # [doc = " `macro_use` prelude that contains macros from `#[macro_use]`'d external crates. Note that"] # [doc = " this contains all kinds of macro, not just `macro_rules!` macro."] # [doc = " ExternCrateId being None implies it being imported from the general prelude import."] macro_use_prelude : FxHashMap < Name , (MacroId , Option < ExternCrateId >) > , # [doc = " Tracks which custom derives are in scope for an item, to allow resolution of derive helper"] # [doc = " attributes."] derive_helpers_in_scope : FxHashMap < AstId < ast :: Item > , Vec < (Name , MacroId , MacroCallId) > > , # [doc = " A mapping from [`hir_expand::MacroDefId`] to [`crate::MacroId`]."] pub macro_def_to_macro_id : FxHashMap < ErasedAstId , MacroId > , # [doc = " The diagnostics that need to be emitted for this crate."] diagnostics : Vec < DefDiagnostic > , # [doc = " The crate data that is shared between a crate's def map and all its block def maps."] data : Arc < DefMapCrateData > , }
    };
}

DefMap!()