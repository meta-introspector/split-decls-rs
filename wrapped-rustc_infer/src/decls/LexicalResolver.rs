macro_rules! deps {
    () => {
        VarInfos!();
        RegionRelations!();
        RegionConstraintData!();
    };
}

macro_rules! LexicalResolver {
    () => {
        deps!();
        struct LexicalResolver < 'cx , 'tcx > { region_rels : & 'cx RegionRelations < 'cx , 'tcx > , var_infos : VarInfos , data : RegionConstraintData < 'tcx > , }
    };
}

LexicalResolver!();