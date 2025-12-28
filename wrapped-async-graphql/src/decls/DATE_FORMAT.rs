macro_rules! DATE_FORMAT {
    () => {
        const DATE_FORMAT : & [FormatItem < '_ >] = format_description ! ("[year]-[month]-[day]") ;
    };
}

DATE_FORMAT!()