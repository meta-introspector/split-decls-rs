macro_rules! deps {
    () => {
        SDLExportOptions!();
    };
}

macro_rules! impl_1012 {
    () => {
        deps!();
        impl Default for SDLExportOptions { fn default () -> Self { Self { sorted_fields : false , sorted_arguments : false , sorted_enum_values : false , federation : false , prefer_single_line_descriptions : false , include_specified_by : false , compose_directive : false , use_space_ident : false , indent_width : 2 , } } }
    };
}

impl_1012!();