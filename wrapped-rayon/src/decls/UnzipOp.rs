macro_rules! deps {
    () => {
        Folder!();
    };
}

macro_rules! UnzipOp {
    () => {
        deps!();
        # [doc = " This trait abstracts the different ways we can \"unzip\" one parallel"] # [doc = " iterator into two distinct consumers, which we can handle almost"] # [doc = " identically apart from how to process the individual items."] trait UnzipOp < T > : Sync + Send { # [doc = " The type of item expected by the left consumer."] type Left : Send ; # [doc = " The type of item expected by the right consumer."] type Right : Send ; # [doc = " Consumes one item and feeds it to one or both of the underlying folders."] fn consume < FA , FB > (& self , item : T , left : FA , right : FB) -> (FA , FB) where FA : Folder < Self :: Left > , FB : Folder < Self :: Right > ; # [doc = " Reports whether this op may support indexed consumers."] # [doc = " - e.g. true for `unzip` where the item count passed through directly."] # [doc = " - e.g. false for `partition` where the sorting is not yet known."] fn indexable () -> bool { false } }
    };
}

UnzipOp!()