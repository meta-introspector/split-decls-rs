macro_rules! deps {
    () => {
        Target!();
        Reference!();
    };
}

macro_rules! convert {
    () => {
        deps!();
        mod convert { use gix_hash :: ObjectId ; use crate :: { raw :: Reference , store_impl :: { file :: loose , packed } , Target , } ; impl From < Reference > for loose :: Reference { fn from (value : Reference) -> Self { loose :: Reference { name : value . name , target : value . target , } } } impl From < loose :: Reference > for Reference { fn from (value : loose :: Reference) -> Self { Reference { name : value . name , target : value . target , peeled : None , } } } impl < 'p > From < packed :: Reference < 'p > > for Reference { fn from (value : packed :: Reference < 'p >) -> Self { Reference { name : value . name . into () , target : Target :: Object (value . target ()) , peeled : value . object . map (| hex | ObjectId :: from_hex (hex) . expect ("parser validation")) , } } } }
    };
}

convert!();