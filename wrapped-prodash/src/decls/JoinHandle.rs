macro_rules! deps {
    () => {
        Event!();
    };
}

macro_rules! JoinHandle {
    () => {
        deps!();
        # [doc = " A handle to the render thread, which when dropped will instruct it to stop showing progress."] pub struct JoinHandle { inner : Option < std :: thread :: JoinHandle < io :: Result < () > > > , connection : std :: sync :: mpsc :: SyncSender < Event > , disconnected : bool , }
    };
}

JoinHandle!();