macro_rules! deps {
    () => {
        Row!();
        Statement!();
    };
}

macro_rules! Rows {
    () => {
        deps!();
        # [doc = " A handle (lazy fallible streaming iterator) for the resulting rows of a query."] # [must_use = "Rows is lazy and will do nothing unless consumed"] pub struct Rows < 'stmt > { pub (crate) stmt : Option < & 'stmt Statement < 'stmt > > , row : Option < Row < 'stmt > > , }
    };
}

Rows!()