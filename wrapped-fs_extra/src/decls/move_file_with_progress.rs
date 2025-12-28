macro_rules! deps {
    () => {
        CopyOptions!();
        Result!();
        TransitProcess!();
    };
}

macro_rules! move_file_with_progress {
    () => {
        deps!();
        # [doc = " Moves a file from one place to another with information about progress."] # [doc = " This function will also copy the permission bits of the original file to the"] # [doc = " destination file."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not limited to just"] # [doc = " these cases:"] # [doc = ""] # [doc = " * This `from` path is not a file."] # [doc = " * This `from` file does not exist."] # [doc = " * The current process does not have the permission to access `from` or write `to`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::file::move_file;"] # [doc = ""] # [doc = " let options = CopyOptions::new(); //Initialize default values for CopyOptions"] # [doc = " let handle = |process_info: TransitProcess|  println!(\"{}\", process_info.total_bytes);"] # [doc = ""] # [doc = " // Move dir1/foo.txt to dir2/foo.txt"] # [doc = " move_file(\"dir1/foo.txt\", \"dir2/foo.txt\", &options, handle)?;"] # [doc = ""] # [doc = " ```"] pub fn move_file_with_progress < P , Q , F > (from : P , to : Q , options : & CopyOptions , progress_handler : F ,) -> Result < u64 > where P : AsRef < Path > , Q : AsRef < Path > , F : FnMut (TransitProcess) , { let mut is_remove = true ; if options . skip_exist && to . as_ref () . exists () && ! options . overwrite { is_remove = false ; } let result = copy_with_progress (& from , to , options , progress_handler) ? ; if is_remove { remove (from) ? ; } Ok (result) }
    };
}

move_file_with_progress!()