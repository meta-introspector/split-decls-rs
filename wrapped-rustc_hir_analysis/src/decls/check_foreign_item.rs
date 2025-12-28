macro_rules! check_foreign_item {
    () => {
        pub (super) fn check_foreign_item < 'tcx > (tcx : TyCtxt < 'tcx > , item : & 'tcx hir :: ForeignItem < 'tcx > ,) -> Result < () , ErrorGuaranteed > { let def_id = item . owner_id . def_id ; debug ! (? item . owner_id , item . name = ? tcx . def_path_str (def_id)) ; match item . kind { hir :: ForeignItemKind :: Fn (sig , ..) => check_item_fn (tcx , def_id , sig . decl) , hir :: ForeignItemKind :: Static (..) | hir :: ForeignItemKind :: Type => Ok (()) , } }
    };
}

check_foreign_item!()