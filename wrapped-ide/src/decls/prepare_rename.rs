macro_rules! deps {
    () => {
        RangeInfo!();
        RenameResult!();
    };
}

macro_rules! prepare_rename {
    () => {
        deps!();
        # [doc = " Prepares a rename. The sole job of this function is to return the TextRange of the thing that is"] # [doc = " being targeted for a rename."] pub (crate) fn prepare_rename (db : & RootDatabase , position : FilePosition ,) -> RenameResult < RangeInfo < () > > { let sema = Semantics :: new (db) ; let source_file = sema . parse_guess_edition (position . file_id) ; let syntax = source_file . syntax () ; let res = find_definitions (& sema , syntax , position , & Name :: new_symbol_root (sym :: underscore)) ? . filter (| (_ , _ , def , _ , _) | def . range_for_rename (& sema) . is_some ()) . map (| (frange , kind , _ , _ , _) | { always ! (frange . range . contains_inclusive (position . offset) && frange . file_id == position . file_id) ; Ok (match kind { SyntaxKind :: LIFETIME => { TextRange :: new (frange . range . start () + TextSize :: from (1) , frange . range . end ()) } _ => frange . range , }) }) . reduce (| acc , cur | match (acc , cur) { (Ok (acc_inner) , Ok (cur_inner)) if acc_inner == cur_inner => Ok (acc_inner) , (e @ Err (_) , _) | (_ , e @ Err (_)) => e , _ => bail ! ("inconsistent text range") , }) ; match res { Some (res) => res . map (| range | RangeInfo :: new (range , ())) , None => bail ! ("No references found at position") , } }
    };
}

prepare_rename!();