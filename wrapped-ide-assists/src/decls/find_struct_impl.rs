macro_rules! deps {
    () => {
        AssistContext!();
    };
}

macro_rules! find_struct_impl {
    () => {
        deps!();
        # [doc = " `find_struct_impl` looks for impl of a struct, but this also has additional feature"] # [doc = " where it takes a list of function names and check if they exist inside impl_, if"] # [doc = " even one match is found, it returns None."] # [doc = ""] # [doc = " That means this function can have 3 potential return values:"] # [doc = "  - `None`: an impl exists, but one of the function names within the impl matches one of the provided names."] # [doc = "  - `Some(None)`: no impl exists."] # [doc = "  - `Some(Some(_))`: an impl exists, with no matching function names."] pub (crate) fn find_struct_impl (ctx : & AssistContext < '_ > , adt : & ast :: Adt , names : & [String] ,) -> Option < Option < ast :: Impl > > { let db = ctx . db () ; let module = adt . syntax () . parent () ? ; let struct_def = ctx . sema . to_def (adt) ? ; let block = module . descendants () . filter_map (ast :: Impl :: cast) . find_map (| impl_blk | { let blk = ctx . sema . to_def (& impl_blk) ? ; let same_ty = match blk . self_ty (db) . as_adt () { Some (def) => def == struct_def , None => false , } ; let not_trait_impl = blk . trait_ (db) . is_none () ; if ! (same_ty && not_trait_impl) { None } else { Some (impl_blk) } }) ; if let Some (ref impl_blk) = block && has_any_fn (impl_blk , names) { return None ; } Some (block) }
    };
}

find_struct_impl!()