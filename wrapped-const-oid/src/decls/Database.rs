macro_rules! deps {
    () => {
        ObjectIdentifier!();
    };
}

macro_rules! Database {
    () => {
        deps!();
        # [doc = " A query interface for OIDs/Names."] # [derive (Copy , Clone)] pub struct Database < 'a > (& 'a [(& 'a ObjectIdentifier , & 'a str)]) ;
    };
}

Database!();