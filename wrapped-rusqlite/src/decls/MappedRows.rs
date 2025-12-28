macro_rules! deps {
    () => {
        Rows!();
    };
}

macro_rules! MappedRows {
    () => {
        deps!();
        # [doc = " An iterator over the mapped resulting rows of a query."] # [doc = ""] # [doc = " `F` is used to transform the _streaming_ iterator into a _standard_"] # [doc = " iterator."] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct MappedRows < 'stmt , F > { rows : Rows < 'stmt > , map : F , }
    };
}

MappedRows!();