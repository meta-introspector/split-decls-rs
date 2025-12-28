macro_rules! DATE_FORMAT {
    () => {
        const DATE_FORMAT : & [FormatItem < '_ >] = format_description ! (version = 2 , "[year]-[month]-[day]") ;
    };
}

DATE_FORMAT!()