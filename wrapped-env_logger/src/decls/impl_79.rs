macro_rules! deps {
    () => {
        ConfigurableFormat!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Default for ConfigurableFormat { fn default () -> Self { Self { timestamp : Some (Default :: default ()) , module_path : false , target : true , level : true , source_file : false , source_line_number : false , indent : Some (4) , suffix : "\n" , # [cfg (feature = "kv")] kv_format : None , } } }
    };
}

impl_79!();