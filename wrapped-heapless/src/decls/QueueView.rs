macro_rules! deps {
    () => {
        QueueInner!();
        ViewStorage!();
        Queue!();
    };
}

macro_rules! QueueView {
    () => {
        deps!();
        # [doc = " A [`Queue`] with dynamic capacity."] # [doc = ""] # [doc = " [`Queue`] coerces to `QueueView`. `QueueView` is `!Sized`, meaning it can only ever be used by"] # [doc = " reference."] pub type QueueView < T > = QueueInner < T , ViewStorage > ;
    };
}

QueueView!();