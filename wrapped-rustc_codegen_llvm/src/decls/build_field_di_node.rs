macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! build_field_di_node {
    () => {
        deps!();
        # [doc = " Creates a `DW_TAG_member` entry inside the DIE represented by the given `type_di_node`."] fn build_field_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , owner : & 'll DIScope , name : & str , layout : TyAndLayout < 'tcx > , offset : Size , flags : DIFlags , type_di_node : & 'll DIType , def_id : Option < DefId > ,) -> & 'll DIType { let (file_metadata , line_number) = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { file_metadata_from_def_id (cx , def_id) } else { (unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER) } ; create_member_type (cx , owner , name , file_metadata , line_number , layout , offset , flags , type_di_node ,) }
    };
}

build_field_di_node!()