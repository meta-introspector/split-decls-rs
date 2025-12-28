macro_rules! FileSystemEdit {
    () => {
        # [derive (Debug , Clone)] pub enum FileSystemEdit { CreateFile { dst : AnchoredPathBuf , initial_contents : String } , MoveFile { src : FileId , dst : AnchoredPathBuf } , MoveDir { src : AnchoredPathBuf , src_id : FileId , dst : AnchoredPathBuf } , }
    };
}

FileSystemEdit!()