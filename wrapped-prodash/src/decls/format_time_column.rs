macro_rules! format_time_column {
    () => {
        fn format_time_column (time : & SystemTime) -> String { format ! ("{}{}" , format_time_for_messages (* time) , VERTICAL_LINE) }
    };
}

format_time_column!();