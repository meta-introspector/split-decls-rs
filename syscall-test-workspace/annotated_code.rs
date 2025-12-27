fn example() {
    #[syscall="write"]
    std::fs::write("test.txt", "data").unwrap();
    #[syscall="read"]
    std::fs::read_to_string("test.txt").unwrap();
}