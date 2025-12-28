macro_rules! MultiUnzip {
    () => {
        # [doc = " An iterator that can be unzipped into multiple collections."] # [doc = ""] # [doc = " See [`.multiunzip()`](crate::Itertools::multiunzip) for more information."] pub trait MultiUnzip < FromI > : Iterator { # [doc = " Unzip this iterator into multiple collections."] fn multiunzip (self) -> FromI ; }
    };
}

MultiUnzip!()